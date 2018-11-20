//! Listing people api is an example API using Actix framework
extern crate listing_people_api_actix;
extern crate actix;
extern crate actix_web;

use listing_people_api_actix::*;
use enpoints::people::model::CreatePerson;
use db::DbExecutor;
use actix::prelude::*;
use actix_web::{AsyncResponder,
    FutureResponse, HttpResponse, Path, State};

/// State with DbEx
struct AppState {
    db: Addr<DbExecutor>,
}

/// Async request handler
fn index((person, state): (Path<CreatePerson>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(CreatePerson {
            name: person.name,
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
    println!("Hello, world!");
}
