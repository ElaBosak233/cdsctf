pub async fn inject(body: &str) -> String {
    body.replace("%TITLE%", &cds_db::get_config().await.meta.title)
}
