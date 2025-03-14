pub mod game_calculator;
pub mod submission_checker;

pub async fn init() -> Result<(), anyhow::Error> {
    game_calculator::init().await;
    submission_checker::init().await;

    Ok(())
}
