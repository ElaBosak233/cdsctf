use nanoid::nanoid;

use crate::{Answer, Captcha, traits::CaptchaError};

pub(crate) async fn generate() -> Result<Captcha, CaptchaError> {
    let challenge = nanoid!(16);

    let captcha = Captcha {
        id: nanoid!(),
        challenge: format!(
            "{}#{}",
            cds_config::get_config().captcha.difficulty,
            challenge
        ),
        criteria: Some(challenge),
    };

    cds_cache::set_ex(&captcha.id, &captcha, 5 * 60).await?;

    Ok(captcha)
}

pub(crate) async fn check(answer: &Answer) -> Result<bool, CaptchaError> {
    let captcha = cds_cache::get::<Captcha>(
        answer
            .id
            .clone()
            .ok_or(CaptchaError::MissingField("id".to_owned()))?,
    )
    .await?
    .ok_or(CaptchaError::Gone())?;

    let challenge = captcha
        .criteria
        .ok_or(CaptchaError::MissingField("criteria".to_owned()))?;

    let mut context = ring::digest::Context::new(&ring::digest::SHA256);
    context.update(answer.content.as_bytes());
    let result = hex::encode(context.finish().as_ref());

    if answer.content.trim().starts_with(challenge.trim())
        && result.starts_with(
            "0".repeat((cds_config::get_config().captcha.difficulty + 1) as usize)
                .as_str(),
        )
    {
        return Ok(true);
    }

    Ok(false)
}
