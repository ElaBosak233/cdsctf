use traits::ICaptcha;

pub mod recaptcha;
pub mod traits;
pub mod turnstile;

pub enum Captcha {
    Recaptcha(recaptcha::Recaptcha),
    Turnstile(turnstile::Turnstile),
}

impl Captcha {
    pub fn new() -> Self {
        return Captcha::Recaptcha(recaptcha::Recaptcha::new());
    }
    pub async fn verify(&self, token: String, client_ip: String) -> bool {
        match self {
            Captcha::Recaptcha(recaptcha) => recaptcha.verify(token, client_ip).await,
            Captcha::Turnstile(turnstile) => turnstile.verify(token, client_ip).await,
        }
    }
}
