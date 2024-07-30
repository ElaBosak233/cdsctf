use async_trait::async_trait;

#[async_trait]
pub trait Captcha {
    async fn verify(&self, token: String, client_ip: String) -> bool;
}
