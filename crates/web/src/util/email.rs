//! Web utility — `email` (shared HTTP helpers).

use scraper::{Html, Selector};

/// Pulls a human-readable title from HTML email bodies.
pub fn extract_title(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("title").ok()?;
    document
        .select(&selector)
        .next()
        .map(|elem| elem.inner_html())
}
