pub fn inject(body: &str) -> String {
    body.replace("%title%", &cds_config::get_variable().meta.title)
}
