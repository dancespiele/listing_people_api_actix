extern crate actix;
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

pub mod db;
pub mod models;
pub mod schema;

pub mod enpoints;