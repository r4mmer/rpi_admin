use envconfig::Envconfig;

#[derive(Clone, Debug, Envconfig)]
pub struct Config {
    /// The connection URL for the database this application should use.
    #[envconfig(from = "DATABASE_FILE", default = "database.db")]
    pub database_file: String,

    /// The port this application should listen on.
    /// Defaults to 8080.
    #[envconfig(from = "LISTEN_ADDR", default = "0.0.0.0:8080")]
    pub listen_addr: String,

    #[envconfig(from = "STATIC_DIR", default = "frontend/build")]
    pub static_dir: String,
}