mod traits;
mod util;
mod worker;

pub use worker::Payload;

use crate::traits::EmailError;

pub async fn init() -> Result<(), EmailError> {
    worker::init().await;

    Ok(())
}
