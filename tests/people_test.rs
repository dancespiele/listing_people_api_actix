#![allow(proc_macro_derive_resolution_fallback)]
extern crate listing_people_api_actix;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate diesel;
extern crate failure;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate uuid;
extern crate futures;
extern crate dotenv;
extern crate listenfd;

use actix::prelude::*;
use listing_people_api_actix::{endpoints, db};
use db::{DbExecutor, AppState};
use actix_web::{http, HttpMessage, test::TestServer};
use endpoints::people::messages::{SendMessage};
use dotenv::dotenv;
use std::{env, str};
use r2d2_diesel::{ConnectionManager};
use r2d2::Pool;
use diesel::pg::PgConnection;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    super_power: bool,
    rich: bool,
    genius: bool
}

fn set_server() -> TestServer {
    dotenv().ok();

    let server = TestServer::build_with_state(|| {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE URL must be set");
        
        let manager = ConnectionManager::<PgConnection>::new(database_url.clone());

        let pool = Pool::builder()
            .build(manager)
            .expect(&format!("Error connecting to {}", database_url));

        let addr = SyncArbiter::start(3, move|| DbExecutor(pool.clone()));

        AppState{db: addr}
    })
    .start(|app| {
        app.resource("/person", |r| {
            r.method(http::Method::GET).with_async(SendMessage::send_get_all);
        });
    });

    server
}

#[test]
fn get_person() {
    let mut srv = set_server();

    let request = srv.client(
        http::Method::GET, "/person").finish().unwrap();

    let response = srv.execute(request.send()).unwrap();
    assert!(response.status().is_success());

    let bytes = srv.execute(response.body()).unwrap();
    let body_string = str::from_utf8(&bytes).unwrap();
    let body: Vec<Person> = serde_json::from_str(&body_string).unwrap();
    let mut index = 0;

    for person in body.iter() {
        match index {
            0 => assert_eq!(person.name, "Carlos"),
            1 => assert_eq!(person.name, "Gauhar"),
            2 => assert_eq!(person.name, "Paco"),
            _ => (),
        }

        index += 1;
    };
}