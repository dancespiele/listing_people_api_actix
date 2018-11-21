//! Listing people api is an example API using Actix framework
extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate uuid;
extern crate actix;
extern crate futures;
extern crate dotenv;

pub mod db;
pub mod models;
pub mod schema;
pub mod enpoints;

use actix::prelude::*;
use enpoints::people::model::CreatePerson;
use db::DbExecutor;
use actix::Actor;
use r2d2_diesel::{ConnectionManager};
use r2d2::Pool;
use futures::Future;
use dotenv::dotenv;
use std::env;
use diesel::pg::PgConnection;
use actix_web::{http, AsyncResponder,
    FutureResponse, HttpResponse, Path, State, server, App};

/// State with DbEx
struct AppState {
    db: Addr<DbExecutor>,
}

/// Async request handler
fn index((person, state): (Path<CreatePerson>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(CreatePerson {
            name: person.name.clone(),
            rich: person.rich,
            super_power: person.super_power,
            genius: person.genius,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(person) => Ok(HttpResponse::Ok().json(person)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn main() {
    dotenv().ok();

    let url = env::var("URL")
        .expect("URL must be set");
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE URL must be set");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder()
        .build(manager)
        .expect(&format!("Error connecting to {}", database_url));

    let addr = SyncArbiter::start(4, || DbExecutor(pool.clone()));
    
    server::new(move || {
        App::with_state(AppState{db: addr.clone()})
            .resource("/person", |r| r.method(http::Method::POST).with(index))
    })
        .bind(url)
        .unwrap()
        .run()
}
