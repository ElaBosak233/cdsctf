use crate::{Answer, Captcha, traits::CaptchaError};

pub(crate) async fn generate() -> Result<Captcha, CaptchaError> {
    let (answer, challenge) = biosvg::BiosvgBuilder::new()
        .length(4)
        .difficulty(cds_config::get_config().captcha.difficulty as u16)
        .colors(vec![
            "#0078D6".to_string(),
            "#aa3333".to_string(),
            "#f08012".to_string(),
            "#33aa00".to_string(),
            "#AA00AA".to_string(),
            "#44CC7F".to_string(),
        ])
        .build()
        .map_err(|err| CaptchaError::BiosvgError)?;
    let id = nanoid::nanoid!();

    let captcha = Captcha {
        id,
        challenge,
        criteria: Some(answer),
    };

    cds_cache::set_ex(format!("captcha:image:{}", &captcha.id), &captcha, 5 * 60).await?;

    Ok(captcha)
}

pub(crate) async fn check(answer: &Answer) -> Result<bool, CaptchaError> {
    let captcha = cds_cache::get_del::<Captcha>(
        format!("captcha:image:{}", answer
            .id
            .clone()
            .ok_or(CaptchaError::MissingField("id".to_owned()))?)
    )
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
