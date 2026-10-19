//! Request spans surround authentication, sessions, rate limits and handlers.

use std::{collections::BTreeMap, time::Instant};

use axum::{
    extract::{FromRequestParts, MatchedPath, RawPathParams, Request},
    http::HeaderValue,
    middleware::Next,
    response::Response,
};
use tracing::Instrument;

pub async fn request(mut req: Request, next: Next) -> Response {
    if req.uri().path() != "/api" && !req.uri().path().starts_with("/api/") {
        return next.run(req).await;
    }

    let request_id = nanoid::nanoid!();
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(|path| path.as_str().to_owned())
        .unwrap_or_else(|| "unmatched".to_owned());
    let client_ip = crate::util::network::get_client_ip(&req);
    let (mut parts, body) = req.into_parts();
    let targets = RawPathParams::from_request_parts(&mut parts, &())
        .await
        .ok()
        .map(|params| {
            params
                .iter()
                .filter(|(name, _)| {
                    name.ends_with("_id") || matches!(*name, "id" | "filename" | "mailbox")
                })
                .map(|(key, value)| (key.to_owned(), value.to_owned()))
                .collect::<BTreeMap<_, _>>()
        })
        .unwrap_or_default();
    req = Request::from_parts(parts, body);
    let span = tracing::info_span!("http.request",
        request_id = %request_id,
        method = %req.method(),
        route = %route,
        targets = ?targets,
        client_ip = ?client_ip,
        operator_kind = "anonymous",
        operator_id = tracing::field::Empty,
        operator_username = tracing::field::Empty,
        auth_method = "anonymous",
        credential_id = tracing::field::Empty,
    );
    // Auth middleware and login handlers update this exact span, even when
    // #[instrument] has entered a child span. No duplicate identity storage.
    req.extensions_mut().insert(span.clone());
    let started = Instant::now();
    let mut response = next.run(req).instrument(span.clone()).await;
    let status = response.status().as_u16();
    span.in_scope(|| {
        tracing::info!(
            status,
            outcome = outcome(status),
            latency_ms = started.elapsed().as_millis() as u64,
            "request completed"
        );
    });
    response
        .headers_mut()
        .insert("x-request-id", HeaderValue::from_str(&request_id).unwrap());
    response
}

fn outcome(status: u16) -> &'static str {
    match status {
        101 | 202 => "accepted",
        200..=299 => "succeeded",
        300..=399 => "redirected",
        400..=499 => "rejected",
        _ => "failed",
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::Write,
        sync::{Arc, Mutex},
    };

    use axum::{
        Router,
        body::{Body, to_bytes},
        extract::{Extension, Path},
        http::{Request as HttpRequest, StatusCode},
        middleware::from_fn,
        response::IntoResponse,
        routing::post,
    };
    use tower::ServiceExt;
    use tracing::instrument::WithSubscriber;

    use super::*;

    #[derive(Clone, Default)]
    struct Capture(Arc<Mutex<Vec<u8>>>);

    impl Write for Capture {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl Capture {
        fn events(&self) -> Vec<serde_json::Value> {
            String::from_utf8(self.0.lock().unwrap().clone())
                .unwrap()
                .lines()
                .map(|line| serde_json::from_str(line).unwrap())
                .collect()
        }
    }

    #[tokio::test]
    async fn concurrent_requests_keep_actor_and_target_distinct_under_child_spans() {
        let captured = Capture::default();
        let writer = captured.clone();
        let subscriber = tracing_subscriber::fmt()
            .json()
            .without_time()
            .with_max_level(tracing::Level::INFO)
            .with_writer(move || writer.clone())
            .finish();
        async fn update(
            Extension(request_span): Extension<tracing::Span>,
            Path(id): Path<i64>,
            body: String,
        ) -> String {
            async {
                // This mimics authentication/login occurring inside a child
                // span: update the explicit request handle, not Span::current.
                request_span.record("operator_kind", "user");
                request_span.record("operator_id", id - 35);
                request_span.record("operator_username", "admin");
                request_span.record("auth_method", "personal_access_token");
                request_span.record("credential_id", "public-pat-id");
                tokio::task::yield_now().await;
                tracing::info!(user_id = id, action = "update", "user updated");
                body
            }
            .instrument(tracing::info_span!("update_user"))
            .await
        }
        let app = Router::new()
            .route("/api/users/{user_id}", post(update))
            .layer(from_fn(request));
        let send = |id| {
            app.clone().oneshot(
                HttpRequest::post(format!("/api/users/{id}?token=query-secret"))
                    .header("x-request-id", "untrusted")
                    .body(Body::from("body-secret"))
                    .unwrap(),
            )
        };
        let (first, second) = async { tokio::join!(send(42), send(43)) }
            .with_subscriber(subscriber)
            .await;
        let first = first.unwrap();
        let second = second.unwrap();
        assert_eq!(first.status(), StatusCode::OK);
        assert_ne!(first.headers()["x-request-id"], "untrusted");
        assert_ne!(
            first.headers()["x-request-id"],
            second.headers()["x-request-id"]
        );
        assert_eq!(
            to_bytes(first.into_body(), 100).await.unwrap(),
            "body-secret"
        );
        let events = captured.events();
        let completed: Vec<_> = events
            .iter()
            .filter(|event| event["fields"]["message"] == "request completed")
            .collect();
        assert_eq!(completed.len(), 2);
        for id in [42, 43] {
            let event = events
                .iter()
                .find(|event| event["fields"]["user_id"] == id)
                .unwrap();
            let root = event["spans"]
                .as_array()
                .unwrap()
                .iter()
                .find(|span| span["name"] == "http.request")
                .unwrap();
            assert_eq!(root["operator_id"], id - 35);
            assert_eq!(root["credential_id"], "public-pat-id");
        }
        for event in completed {
            assert_eq!(event["fields"]["status"], 200);
            assert_eq!(event["fields"]["outcome"], "succeeded");
            assert_eq!(event["span"]["operator_username"], "admin");
        }
        let logs = serde_json::to_string(&events).unwrap();
        assert!(!logs.contains("query-secret"));
        assert!(!logs.contains("body-secret"));
        assert!(!logs.contains("untrusted"));
    }

    #[tokio::test]
    async fn rejections_before_the_handler_still_have_request_span_and_completion() {
        let captured = Capture::default();
        let writer = captured.clone();
        let subscriber = tracing_subscriber::fmt()
            .json()
            .without_time()
            .with_max_level(tracing::Level::INFO)
            .with_writer(move || writer.clone())
            .finish();
        let app = Router::new()
            .route(
                "/api/private",
                post(|| async {
                    panic!("handler must not run");
                }),
            )
            .layer(from_fn(|_: Request, _: Next| async {
                crate::traits::WebError::Unauthorized(serde_json::json!("unauthorized"))
                    .into_response()
            }))
            .layer(from_fn(request));
        let response = app
            .oneshot(
                HttpRequest::post("/api/private")
                    .body(Body::empty())
                    .unwrap(),
            )
            .with_subscriber(subscriber)
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let events = captured.events();
        let completed = events
            .iter()
            .find(|event| event["fields"]["message"] == "request completed")
            .unwrap();
        assert_eq!(completed["fields"]["outcome"], "rejected");
        assert_eq!(completed["span"]["operator_kind"], "anonymous");
        assert!(
            events
                .iter()
                .any(|event| event["fields"]["error_code"] == "unauthorized")
        );
    }

    #[test]
    fn response_outcomes_distinguish_acceptance_from_completion() {
        assert_eq!(outcome(101), "accepted");
        assert_eq!(outcome(202), "accepted");
        assert_eq!(outcome(200), "succeeded");
        assert_eq!(outcome(302), "redirected");
        assert_eq!(outcome(403), "rejected");
        assert_eq!(outcome(500), "failed");
    }
}
