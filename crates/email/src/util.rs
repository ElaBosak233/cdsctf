pub async fn inject(body: &str) -> String {
    body.replace("%title%", &cds_db::get_config().await.meta.title)
}
