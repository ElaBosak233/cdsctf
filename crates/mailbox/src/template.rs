//! Shared email layout and localized copy for account messages.

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use uuid::Uuid;

use crate::Payload;

#[derive(Clone, Copy)]
pub enum EmailPurpose {
    Verify,
    Reset,
}

#[derive(Deserialize)]
struct Copy {
    greeting: String,
    code_label: String,
    expiry: String,
    ignore: String,
    verify: PurposeCopy,
    reset: PurposeCopy,
}

#[derive(Deserialize)]
struct PurposeCopy {
    heading: String,
    instruction: String,
}

impl Payload {
    /// Renders a verification or password-reset message for the mail queue.
    pub fn transactional(
        name: String,
        email: String,
        purpose: EmailPurpose,
        locale: Option<&str>,
        code: &str,
        platform: &str,
    ) -> Result<Self> {
        let locale = match locale.unwrap_or_default().to_ascii_lowercase().as_str() {
            "zh" | "zh-cn" | "zh-hans" => "zh-CN",
            "zh-tw" | "zh-hk" | "zh-mo" | "zh-hant" => "zh-TW",
            "ja" | "ja-jp" => "ja-JP",
            _ => "en-US",
        };
        let copy_path = format!("mail/{locale}.json");
        let copy = cds_assets::Embeds::get(&copy_path)
            .with_context(|| format!("missing mail copy: {copy_path}"))?;
        let copy: Copy = serde_json::from_slice(&copy.data)
            .with_context(|| format!("invalid mail copy: {copy_path}"))?;
        let message = match purpose {
            EmailPurpose::Verify => copy.verify,
            EmailPurpose::Reset => copy.reset,
        };
        let greeting = copy.greeting.replace("{name}", &name);
        let subject = format!("{} · {platform}", message.heading);
        let logo_content_id = format!("{}@cdsctf", Uuid::new_v4());
        let plain = format!(
            "{platform}\n{}\n\n{greeting}\n{}\n\n{}\n{code}\n{}\n\n{}",
            message.heading, message.instruction, copy.code_label, copy.expiry, copy.ignore,
        );

        let layout = cds_assets::Embeds::get("mail/layout.html").context("missing mail layout")?;
        let layout = std::str::from_utf8(&layout.data).context("invalid mail layout encoding")?;
        let mut sections = layout.split("{{");
        let mut html = sections.next().unwrap_or_default().to_owned();

        for section in sections {
            let (key, rest) = section
                .split_once("}}")
                .context("unclosed mail placeholder")?;
            let value = match key {
                "locale" => locale,
                "subject" => &subject,
                "logo" => &logo_content_id,
                "platform" => platform,
                "heading" => &message.heading,
                "greeting" => &greeting,
                "instruction" => &message.instruction,
                "code_label" => &copy.code_label,
                "code" => code,
                "expiry" => &copy.expiry,
                "ignore" => &copy.ignore,
                _ => bail!("unknown mail placeholder: {key}"),
            };
            for ch in value.chars() {
                match ch {
                    '&' => html.push_str("&amp;"),
                    '<' => html.push_str("&lt;"),
                    '>' => html.push_str("&gt;"),
                    '"' => html.push_str("&quot;"),
                    '\'' => html.push_str("&#39;"),
                    _ => html.push(ch),
                }
            }
            html.push_str(rest);
        }

        Ok(Self {
            name,
            email,
            subject,
            html,
            plain,
            logo_content_id,
        })
    }
}
