use async_trait::async_trait;

#[async_trait]
pub trait Captcha: Send + Sync {
    async fn verify(&self, token: String, client_ip: String) -> bool;
}
