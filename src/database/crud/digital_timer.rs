use sqlx::SqlitePool;

use crate::database::models::digital_timer::{DigitalTimerFromDB, DigitalTimer};
use crate::errors::Error;

pub struct DigitalTimerCRUD;

impl DigitalTimerCRUD {

    pub async fn create_table(pool: &SqlitePool) -> Result<(), Error> {
        sqlx::query("
            CREATE TABLE digital_timer (
            id INTEGER PRIMARY KEY,
            name VARCHAR(60) NOT NULL,
            pin UNSIGNED TINYINT NOT NULL,
            hour UNSIGNED TINYINT NOT NULL,
            minute UNSIGNED TINYINT NOT NULL,
            duration UNSIGNED INTEGER NOT NULL,
            is_enabled BOOLEAN DEFAULT FALSE NOT NULL
        )")
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<DigitalTimerFromDB>, Error> {
        let digital_timers = sqlx::query_as!(
            DigitalTimerFromDB,
            r#"
            SELECT id, name, pin, hour, minute, duration, is_enabled
            FROM digital_timer
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(digital_timers)
    }

    pub async fn get_one(pool: &SqlitePool, id: i64) -> Result<DigitalTimerFromDB, Error> {
        let digital_timer = sqlx::query_as!(
            DigitalTimerFromDB,
            r#"
            SELECT id, name, pin, hour, minute, duration, is_enabled
            FROM digital_timer
            WHERE id = ?
            "#,
            id,
        )
        .fetch_one(pool)
        .await?;

        Ok(digital_timer)
    }

    pub async fn insert_one(
        pool: &SqlitePool,
        timer: &DigitalTimer,
    ) -> Result<i64, Error> {
        let id = sqlx::query!(
            r#"
            INSERT INTO digital_timer (name, pin, hour, minute, duration, is_enabled)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            timer.name,
            timer.pin,
            timer.hour,
            timer.minute,
            timer.duration,
            timer.is_enabled,
        )
        .execute(pool)
        .await?.last_insert_rowid();

        Ok(id)
    }

    pub async fn update_one(
        pool: &SqlitePool,
        timer: &DigitalTimerFromDB,
    ) -> Result<(), Error> {
        sqlx::query!(
            r#"
            UPDATE digital_timer
            SET name = ?, pin = ?, hour = ?, minute = ?, duration = ?, is_enabled = ?
            WHERE id = ?
            "#,
            timer.name,
            timer.pin,
            timer.hour,
            timer.minute,
            timer.duration,
            timer.is_enabled,
            timer.id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_one(pool: &SqlitePool, id: i64) -> Result<(), Error> {
        sqlx::query!(
            r#"
            DELETE FROM digital_timer
            WHERE id = ?
            "#,
            id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_current_timers(pool: &SqlitePool) -> Result<Vec<DigitalTimerFromDB>, Error> {
        let digital_timers = sqlx::query_as!(
            DigitalTimerFromDB,
            r#"
            SELECT id, name, pin, hour, minute, duration, is_enabled
            FROM digital_timer
            WHERE is_enabled = TRUE
                AND hour = strftime('%H', 'now')
                AND minute = strftime('%M', 'now')
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(digital_timers)
    }
}