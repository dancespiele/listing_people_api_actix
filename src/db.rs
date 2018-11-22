use diesel::PgConnection;
use r2d2::Pool;
use r2d2_diesel::{ConnectionManager};
use actix::prelude::*;

/// State with DbEx
pub struct AppState {
    pub db: Addr<DbExecutor>,
}


/// Db executor actor with Postgres connection
pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);