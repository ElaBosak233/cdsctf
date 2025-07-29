mod traits;
mod util;
mod worker;

use lettre::AsyncTransport;
pub use worker::Payload;

use crate::traits::EmailError;

pub async fn init() -> Result<(), EmailError> {
    worker::init().await;

    Ok(())
}
