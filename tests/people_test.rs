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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    super_power: bool,
    rich: bool,
    genius: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct People{ 
    list: Vec<Person>
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
        app.resource("/people", |r| {
            r.method(http::Method::GET).with_async(SendMessage::send_get_all);
            r.method(http::Method::POST).with_async(SendMessage::send_create);
        });
        app.resource("/person/{name}", |r| {
            r.method(http::Method::GET).with_async(SendMessage::send_get_person);
            r.method(http::Method::DELETE).with_async(SendMessage::send_delete);
        });
    });

    server
}

#[test]
fn a_add_people() {
    let mut srv = set_server();

    let people: People = People {
        list: vec![
            Person {
                name: String::from("person1"),
                super_power: true,
                rich: false,
                genius: false,
            },
            Person {
                name: String::from("person2"),
                super_power: false,
                rich: true, 
                genius: false,
            }
        ],
    };

    let request = srv.client(http::Method::POST, "/people").json(people).unwrap();

    let response = srv.execute(request.send()).unwrap();
    assert!(response.status().is_success());

    let bytes = srv.execute(response.body()).unwrap();
    let body_string = str::from_utf8(&bytes).unwrap();
    let body: Vec<Person> = serde_json::from_str(body_string).unwrap();

    assert_eq!(body, vec![
        Person {
            name: String::from("person1"),
            super_power: true,
            rich: false,
            genius: false,
        },
        Person {
            name: String::from("person2"),
            super_power: false,
            rich: true, 
            genius: false,
        }
    ])
}

#[test]
fn b_get_people() {
    let mut srv = set_server();

    let request = srv.client(
        http::Method::GET, "/people").finish().unwrap();

    let response = srv.execute(request.send()).unwrap();
    assert!(response.status().is_success());

    let bytes = srv.execute(response.body()).unwrap();
    let body_string = str::from_utf8(&bytes).unwrap();
    let body: Vec<Person> = serde_json::from_str(&body_string).unwrap();

    assert_eq!(body, vec![
        Person {
            name: String::from("person1"),
            super_power: true,
            rich: false,
            genius: false,
        },
        Person {
            name: String::from("person2"),
            super_power: false,
            rich: true, 
            genius: false,
        }
    ])
}

#[test]
fn c_get_person() {
    let mut srv = set_server();

    let request = srv.client(
        http::Method::GET, "/person/person1").finish().unwrap();

    let response = srv.execute(request.send()).unwrap();
    assert!(response.status().is_success());

    let bytes = srv.execute(response.body()).unwrap();

    let body_string = str::from_utf8(&bytes).unwrap();

    let body: Person = serde_json::from_str(body_string).unwrap();

    assert_eq!(body, Person {
            name: String::from("person1"),
            super_power: true,
            rich: false,
            genius: false,
    });
}

#[test]
fn d_delete_person() {
    let mut srv = set_server();

    let people: Vec<String>= vec![String::from("person1"), String::from("person2")];

    for person in people.iter() {
        let person_fmt = format!("/person/{}", person);
        let request = srv.client(
        http::Method::DELETE, &person_fmt).finish().unwrap();

        let response = srv.execute(request.send()).unwrap();
        assert!(response.status().is_success());

        let bytes = srv.execute(response.body()).unwrap();

        let body_string = str::from_utf8(&bytes).unwrap();

        let body: &str = serde_json::from_str(body_string).unwrap();

        let person_to_test = format!("{} was deleted successfully from the database", person);
        assert_eq!(body, &person_to_test);
    }
}

#[test]
fn f_get_person_after_delete() {
    let mut srv = set_server();

    let request = srv.client(
        http::Method::GET, "/person/person1").finish().unwrap();

    let response = srv.execute(request.send()).unwrap();
    assert_eq!(response.status().as_u16(), 404);

    let bytes = srv.execute(response.body()).unwrap();

    let body_string = str::from_utf8(&bytes).unwrap();

    let body: &str = serde_json::from_str(body_string).unwrap();

    assert_eq!(body, "The person person1 doesn't exist in the database");
}