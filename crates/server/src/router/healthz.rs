//! `/healthz`：OpenAPI 仅描述 GET；若需与其它 method 对齐，可再挂 `route`。

#[utoipa::path(
    get,
    path = "/healthz",
    tag = "system",
    responses((status = 200, description = "Plain-text liveness response"))
)]
pub async fn healthz() -> &'static str {
    "Ok"
}
