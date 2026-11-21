//! Outbound email over SMTP using settings stored in the database.
//!
//! HTTP handlers publish a JSON [`Payload`] to `cds.mail.send`. The worker
//! consumes it and delivers the message through [`Mailbox`].

mod template;
mod traits;

use std::{io::Cursor, time::Duration};

use anyhow::{Context, anyhow};
use cds_db::{Config, DB, config::email};
use cds_media::Media;
use image::{ImageFormat, RgbaImage};
use lettre::{
    Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Attachment, Mailbox as LMailbox, MultiPart, SinglePart, header::ContentType},
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
};
use serde::{Deserialize, Serialize};

pub use crate::template::EmailPurpose;
use crate::traits::MailboxError;

/// Validity of verification and reset codes issued by the platform.
pub const CODE_TTL: Duration = Duration::from_secs(60 * 60);

/// JSON envelope for one outbound message on `cds.mail.send`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub name: String,
    pub email: String,
    subject: String,
    html: String,
    plain: String,
    logo_content_id: String,
}

/// SMTP mailer backed by live platform settings and media storage.
#[derive(Clone)]
pub struct Mailbox {
    db: DB,
    media: Media,
}

impl Mailbox {
    pub fn new(db: DB, media: Media) -> Self {
        Self { db, media }
    }

    /// Sends one message described by a queue [`Payload`].
    pub async fn send_payload(&self, payload: &Payload) -> Result<(), MailboxError> {
        let config = cds_db::get_config(&self.db.conn).await;
        let mailer = Self::get_mailer(&config.email)?;
        let logo = self.get_logo(config.logo_hash.as_deref()).await?;

        mailer.send(build_message(&config, payload, logo)?).await?;
        Ok(())
    }

    async fn get_logo(&self, hash: Option<&str>) -> Result<Vec<u8>, MailboxError> {
        if let Some(hash) = hash
            && let Ok(Ok(data)) = tokio::time::timeout(
                Duration::from_secs(3),
                self.media.get("media".to_owned(), hash.to_owned()),
            )
            .await
            && let Ok(Ok(png)) = tokio::task::spawn_blocking(move || logo_to_png(&data)).await
        {
            return Ok(png);
        }

        // Rasterized from the frontend's web/public/logo.svg for email clients.
        Ok(cds_assets::Embeds::get("mail/logo.png")
            .context("missing default mail logo")?
            .data
            .into_owned())
    }

    fn get_mailer(
        config: &email::Config,
    ) -> Result<AsyncSmtpTransport<Tokio1Executor>, MailboxError> {
        if !config.enabled {
            return Err(MailboxError::OtherError(anyhow!("disabled")));
        }

        let credentials = Credentials::new(config.username.clone(), config.password.clone());
        let builder = match config.tls {
            email::Tls::Starttls => {
                AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)
            }
            email::Tls::Tls => Ok(
                AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)?.tls(Tls::Wrapper(
                    TlsParameters::builder(config.host.clone()).build()?,
                )),
            ),
            email::Tls::None => Ok(AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(
                &config.host,
            )),
        }?;

        Ok(builder
            .port(config.port)
            .credentials(credentials)
            .timeout(Some(Duration::from_secs(10)))
            .build())
    }
}

fn logo_to_png(data: &[u8]) -> image::ImageResult<Vec<u8>> {
    const SIZE: u32 = 96;

    let thumbnail = image::load_from_memory_with_format(data, ImageFormat::WebP)?
        .thumbnail(SIZE, SIZE)
        .to_rgba8();
    let mut canvas = RgbaImage::new(SIZE, SIZE);
    image::imageops::overlay(
        &mut canvas,
        &thumbnail,
        i64::from((SIZE - thumbnail.width()) / 2),
        i64::from((SIZE - thumbnail.height()) / 2),
    );

    let mut png = Cursor::new(Vec::new());
    canvas.write_to(&mut png, ImageFormat::Png)?;
    Ok(png.into_inner())
}

fn build_message(
    config: &Config,
    payload: &Payload,
    logo: Vec<u8>,
) -> Result<Message, MailboxError> {
    let envelope = Message::builder()
        .from(LMailbox::new(
            Some(config.meta.title.clone()),
            config.email.username.parse::<Address>()?,
        ))
        .to(LMailbox::new(
            Some(payload.name.clone()),
            payload.email.parse::<Address>()?,
        ));

    let logo_type = ContentType::parse("image/png").context("invalid mail logo content type")?;
    let related_type = ContentType::parse(&format!(
        "multipart/related; type=\"text/html\"; boundary=\"{}\"",
        uuid::Uuid::new_v4()
    ))
    .context("invalid related mail content type")?;
    let html = MultiPart::related()
        .header(related_type)
        .singlepart(SinglePart::html(payload.html.clone()))
        .singlepart(Attachment::new_inline(payload.logo_content_id.clone()).body(logo, logo_type));
    let multipart = MultiPart::alternative()
        .singlepart(SinglePart::plain(payload.plain.clone()))
        .multipart(html);

    Ok(envelope
        .subject(payload.subject.clone())
        .multipart(multipart)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CODE: &str = "K8Z_aEHXqS-37NmA5pL24";

    #[test]
    fn renders_both_purposes_in_all_supported_languages() {
        for (requested, expected) in [
            ("en-US", "en-US"),
            ("zh-CN", "zh-CN"),
            ("zh-TW", "zh-TW"),
            ("ja-JP", "ja-JP"),
            ("unsupported", "en-US"),
        ] {
            for purpose in [EmailPurpose::Verify, EmailPurpose::Reset] {
                let payload = Payload::transactional(
                    "A < B & {{code}}".into(),
                    "recipient@example.org".into(),
                    purpose,
                    Some(requested),
                    CODE,
                    "Test & Learn",
                )
                .unwrap();
                assert!(payload.subject.ends_with(" · Test & Learn"));
                assert!(
                    payload
                        .html
                        .contains(&format!("<html lang=\"{expected}\">"))
                );
                assert!(payload.html.contains("A &lt; B &amp; {{code}}"));
                assert!(payload.html.contains("Test &amp; Learn"));
                assert!(payload.html.contains(CODE));
                assert!(
                    payload
                        .html
                        .contains(&format!("cid:{}", payload.logo_content_id))
                );
                assert!(payload.plain.contains("A < B & {{code}}"));
                assert!(payload.plain.contains(&format!("\n{CODE}\n")));
            }
        }
    }

    #[test]
    fn builds_plain_text_and_related_html_with_inline_png() {
        let mut config = Config::default();
        config.email.username = "sender@example.org".into();
        let payload = Payload::transactional(
            "Ela".into(),
            "recipient@example.org".into(),
            EmailPurpose::Verify,
            None,
            CODE,
            &config.meta.title,
        )
        .unwrap();
        let logo = cds_assets::Embeds::get("mail/logo.png").unwrap();
        let message = build_message(&config, &payload, logo.data.into_owned()).unwrap();
        let message = String::from_utf8(message.formatted()).unwrap();
        let content_id = &payload.logo_content_id;
        let plain = message.find("Content-Type: text/plain").unwrap();
        let related = message.find("Content-Type: multipart/related").unwrap();
        let html = message.find("Content-Type: text/html").unwrap();
        let image = message.find("Content-Type: image/png").unwrap();

        assert!(message.contains("Content-Type: multipart/alternative"));
        assert!(plain < related && related < html && html < image);
        assert!(message.contains("Content-Disposition: inline"));
        assert!(message.contains(&format!("Content-ID: <{content_id}>")));
    }

    #[test]
    fn converts_custom_webp_logo_to_square_png_without_stretching() {
        let mut webp = Cursor::new(Vec::new());
        RgbaImage::from_pixel(120, 40, image::Rgba([0, 125, 185, 255]))
            .write_to(&mut webp, ImageFormat::WebP)
            .unwrap();
        let png = logo_to_png(webp.get_ref()).unwrap();
        let png = image::load_from_memory_with_format(&png, ImageFormat::Png)
            .unwrap()
            .to_rgba8();

        assert_eq!(png.dimensions(), (96, 96));
        assert_eq!(png.get_pixel(0, 0).0[3], 0);
        assert_eq!(png.get_pixel(0, 48).0[3], 255);
    }
}
