//! Captcha backend — `pow` (verification / challenge generation).

use nanoid::nanoid;

use crate::{Answer, Captcha, CaptchaChallenge, traits::CaptchaError};

/// Produces captcha challenges or dynamic checker environment data.
pub(crate) async fn generate(c: &Captcha) -> Result<CaptchaChallenge, CaptchaError> {
    let challenge = nanoid!(16);

    let captcha = CaptchaChallenge {
        id: nanoid!(),
        challenge: format!(
            "{}#{}",
            cds_db::get_config(&c.db.conn).await.captcha.difficulty,
            challenge
        ),
        criteria: Some(challenge),
    };

    let _ = &c
        .cache
        .set_ex(format!("captcha:pow:{}", &captcha.id), &captcha, 5 * 60)
        .await?;

    Ok(captcha)
}

/// Verifies a submitted flag against the checker script.
pub(crate) async fn check(c: &Captcha, answer: &Answer) -> Result<bool, CaptchaError> {
    let captcha = c
        .cache
        .get_del::<CaptchaChallenge>(format!(
            "captcha:pow:{}",
            answer
                .id
                .clone()
                .ok_or(CaptchaError::MissingField("id".to_owned()))?
        ))
        .await?
        .ok_or(CaptchaError::Gone)?;

    let challenge = captcha
        .criteria
        .ok_or(CaptchaError::MissingField("criteria".to_owned()))?;

    let mut context = ring::digest::Context::new(&ring::digest::SHA256);
    context.update(answer.content.as_bytes());
    let result = hex::encode(context.finish().as_ref());

    if answer.content.trim().starts_with(challenge.trim())
        && result.starts_with(
            "0".repeat((cds_db::get_config(&c.db.conn).await.captcha.difficulty + 1) as usize)
                .as_str(),
        )
    {
        return Ok(true);
    }

    Ok(false)
}
