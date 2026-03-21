use std::sync::Arc;

use crate::traits::AppState;

pub mod calculator;
pub mod game_calculator;
pub mod submission_checker;

pub async fn init(s: Arc<AppState>) -> Result<(), anyhow::Error> {
    game_calculator::init(s.clone()).await;
    submission_checker::init(s).await;

    Ok(())
}
