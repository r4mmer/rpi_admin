pub mod digital_timer;

pub use digital_timer::DigitalTimerCRUD;
use crate::errors::Error;
use sqlx::SqlitePool;

pub async fn init_tables(pool: &SqlitePool) -> Result<(), Error> {
    digital_timer::DigitalTimerCRUD::create_table(pool).await?;
    Ok(())
}