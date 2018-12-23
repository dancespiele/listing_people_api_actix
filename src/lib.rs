#![allow(proc_macro_derive_resolution_fallback)]
//! Listing people api is an example API using Actix framework
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate juniper;
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
extern crate http;

pub mod db;
pub mod models;
pub mod schema;
pub mod endpoints;
pub mod error;
pub mod middlewares;