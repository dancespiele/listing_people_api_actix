#![allow(proc_macro_derive_resolution_fallback)]
//! Listing people api is an example API using Actix framework
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate uuid;
extern crate futures;
extern crate dotenv;
extern crate listenfd;

mod db;
mod models;
mod schema;
mod enpoints;
mod error;

use listenfd::ListenFd;
use actix::prelude::*;
use enpoints::routes::routes;
use db::{DbExecutor};

use r2d2_diesel::{ConnectionManager};
use r2d2::Pool;

use dotenv::dotenv;
use std::env;
use diesel::pg::PgConnection;
use actix_web::{server};

fn main() {
    dotenv().ok();

    let mut listenfd = ListenFd::from_env();

    ::std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let sys = actix::System::new("listing-people-api-actix");

    let url = env::var("URL")
        .expect("URL must be set");
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE URL must be set");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url.clone());

    let pool = Pool::builder()
        .build(manager)
        .expect(&format!("Error connecting to {}", database_url));

    let addr = SyncArbiter::start(3, move|| DbExecutor(pool.clone()));
    
    let mut server = server::new(move || routes(addr.clone()))
        .workers(4);

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind(url).unwrap()
    };

    server.start();
    
    let _= sys.run();
}
