//! `/healthz`：纯 Axum 路由，不参与 OpenAPI 汇总。

pub async fn healthz() -> &'static str {
    "Ok"
}
