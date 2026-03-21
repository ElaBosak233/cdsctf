//! Captcha backend — `image` (verification / challenge generation).

use crate::{Answer, Captcha, CaptchaChallenge, traits::CaptchaError};

/// Produces captcha challenges or dynamic checker environment data.
pub(crate) async fn generate(c: &Captcha) -> Result<CaptchaChallenge, CaptchaError> {
    let (answer, challenge) = biosvg::BiosvgBuilder::new()
        .length(4)
        .difficulty(cds_db::get_config(&c.db.conn).await.captcha.difficulty as u16)
        .colors(vec![
            "#0078D6".to_string(),
            "#aa3333".to_string(),
            "#f08012".to_string(),
            "#33aa00".to_string(),
            "#AA00AA".to_string(),
            "#44CC7F".to_string(),
        ])
        .build()
        .map_err(|_err| CaptchaError::BiosvgError)?;
    let id = nanoid::nanoid!();

    let captcha = CaptchaChallenge {
        id,
        challenge,
        criteria: Some(answer),
    };

    c.cache
        .set_ex(format!("captcha:image:{}", &captcha.id), &captcha, 5 * 60)
        .await?;

    Ok(captcha)
}

/// Verifies a submitted flag against the checker script.
pub(crate) async fn check(c: &Captcha, answer: &Answer) -> Result<bool, CaptchaError> {
    let captcha = c
        .cache
        .get_del::<CaptchaChallenge>(format!(
            "captcha:image:{}",
            answer
                .id
                .clone()
                .ok_or(CaptchaError::MissingField("id".to_owned()))?
        ))
        .await?
        .ok_or(CaptchaError::Gone)?;

    let criteria = captcha
        .criteria
        .ok_or(CaptchaError::MissingField("criteria".to_owned()))?;

    if answer.content.trim().to_lowercase() == criteria.trim().to_lowercase() {
        return Ok(true);
    }

    Ok(false)
}
