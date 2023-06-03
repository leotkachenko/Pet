mod commands {
    pub mod create {
        pub mod controller;
        pub mod service;
    }
}
mod dto {
    pub mod request;
}
mod db {
    pub mod models;
}
pub use commands::create::controller;
use sqlx::{Pool, Postgres};

pub struct AppState {
    pub db: Pool<Postgres>,
}