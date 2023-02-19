mod digital_timer;

use std::collections::HashMap;

use sqlx::SqlitePool;
use tokio::time;

const TIMER_INTERVAL: u64 = 30;

pub async fn run(pool: SqlitePool) {
    let mut interval = time::interval(time::Duration::from_secs(TIMER_INTERVAL));

    let mut digital_timers: HashMap<i64, i64> = HashMap::new();
    let digital_timer_pool = pool.clone();

    let mut it: i64 = 0;
    loop {
        // Waits for the next tick.
        interval.tick().await;

        // If we want multiple concurrent workers we can `join` them here and await the result.
        digital_timer::worker(&digital_timer_pool, &mut digital_timers, it).await;

        dbg!(&digital_timers);
        // Increase iteration index
        it += 1;
    }
}