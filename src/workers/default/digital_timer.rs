use crate::database::crud;

use std::collections::HashMap;

use tracing::{event, Level};
use sqlx::SqlitePool;

const N_SKIP_FOR_ITER: i64 = 10;

pub async fn worker(pool: &SqlitePool, last_timers: &mut HashMap<i64, i64>, iteration: i64) {
    event!(Level::INFO, "Running a tick for digital timers");

    // Clean old timers
    let mut to_remove: Vec<i64> = Vec::new();
    for (id, iter) in last_timers.iter() {
        if *iter < (iteration - N_SKIP_FOR_ITER) {
            to_remove.push(*id);
        }
    }
    for id in to_remove {
        last_timers.remove(&id);
    }

    // Fetch all timers for this minute
    let current_timers = crud::DigitalTimerCRUD::get_current_timers(pool).await;
    // let current_timers = crud::DigitalTimerCRUD::get_all(pool).await;
    match current_timers {
        Ok(timers) => {
            for timer in timers {
                if last_timers.contains_key(&timer.id) {
                    event!(Level::INFO, "This timer has already been triggered in the last 10 minutes");
                    continue;
                }
                last_timers.insert(timer.id, iteration);
                event!(Level::INFO, "Timer: {:?}", timer);
                event!(Level::INFO, "Enabling pin {:?} for {:?} seconds", timer.pin, timer.duration);
            }
        }
        Err(err) => {
            panic!("Error: {:?}", err);
        }
    }
}