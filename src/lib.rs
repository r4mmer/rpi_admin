pub mod errors;
pub mod database;
pub mod http;
pub mod config;

#[cfg(target_arch = "armv7-unknown-linux-gnueabihf")]
#[path = "workers/rpi/worker.rs"]
pub mod worker;

#[cfg(not(target_arch = "armv7-unknown-linux-gnueabihf"))]
#[path = "workers/default/worker.rs"]
pub mod worker;