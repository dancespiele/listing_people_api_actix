use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::{ConnectionManager};
use actix::prelude::*;
use endpoints::graphql::{Schema};
use std::ops::Deref;

pub type StartPool = Pool<ConnectionManager<PgConnection>>;

/// State with DbEx
pub struct AppState {
    pub db: Addr<DbExecutor>,
}

///Start a Pool
pub fn init_pool(url: String) -> StartPool {
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::new(manager).expect("db pool")
}


/// Db executor actor with Postgres connection
pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

pub struct GraphQLExecutor {
    pub schema: std::sync::Arc<Schema>,
    pub pool: StartPool,
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

impl GraphQLExecutor {
    pub fn new(schema: std::sync::Arc<Schema>, pool: StartPool) -> GraphQLExecutor {
        GraphQLExecutor { schema, pool }
    }
}

/// State with GraphQl
pub struct GraphQLState {
    pub executor: Addr<GraphQLExecutor>,
}

/// Connection to db for graph
pub struct Conn(pub PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for Conn {
    type Target = PgConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}