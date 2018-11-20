use diesel::PgConnection;
use r2d2::Pool;
use r2d2_diesel::{ConnectionManager};

/// Db executor actor with Postgres connection
pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);