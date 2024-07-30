pub fn get_content_type(ext: &str) -> mime::Mime {
    match ext.to_lowercase().as_str() {
        "html" => mime::TEXT_HTML,
        "css" => mime::TEXT_CSS,
        "js" => mime::APPLICATION_JAVASCRIPT,
        "png" => mime::IMAGE_PNG,
        "jpg" | "jpeg" => mime::IMAGE_JPEG,
        "gif" => mime::IMAGE_GIF,
        "svg" => mime::IMAGE_SVG,
        "json" => mime::APPLICATION_JSON,
        "xml" => mime::TEXT_XML,
        "txt" => mime::TEXT_PLAIN,
        "woff" => mime::FONT_WOFF,
        "woff2" => mime::FONT_WOFF2,
        _ => mime::APPLICATION_OCTET_STREAM,
    }
}
