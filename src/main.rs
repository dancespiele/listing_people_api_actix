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
extern crate r2d2;
extern crate r2d2_diesel;
extern crate uuid;
extern crate futures;
extern crate dotenv;

pub mod db;
pub mod models;
pub mod schema;
pub mod enpoints;

use actix::prelude::*;
use enpoints::people::routes::{insert_person, get_person};
use db::{DbExecutor, AppState};

use r2d2_diesel::{ConnectionManager};
use r2d2::Pool;

use dotenv::dotenv;
use std::env;
use diesel::pg::PgConnection;
use actix_web::{http,server, App, middleware};

fn main() {
    dotenv().ok();

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
    
    server::new(move || {
        App::with_state(AppState{db: addr.clone()})
            .middleware(middleware::Logger::default())
            .resource("/person", |r| r.method(http::Method::POST).with(insert_person))
            .resource("/person/{name}", |r| r.method(http::Method::GET).with(get_person))})

        .workers(4)
        .bind(url)
        .unwrap()
        .start();
    
    let _= sys.run();
}
