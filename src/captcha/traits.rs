pub trait ICaptcha {
    fn new() -> Self;
    fn verify(
        &self,
        token: String,
        client_ip: String,
    ) -> impl std::future::Future<Output = bool> + Send;
}
