//! people model
//! use actix::prelude::*;
use actix_web::*;
use actix::prelude::*;
use diesel;
use diesel::prelude::*;
use db::DbExecutor;
use uuid;

use models;
use schema;

pub struct CreatePerson {
    pub name: String,
    pub super_power: bool,
    pub rich: bool,
    pub genius: bool
}

/// Message for every person to crate
impl Message for CreatePerson {
    type Result = Result<models::Person, Error>;
}

/// Implement Actor
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<CreatePerson> for DbExecutor {
    type Result = Result<models::Person, Error>;

    fn handle(&mut self, msg: CreatePerson, _: &mut Self::Context) -> Self::Result {
        use self::schema::people::dsl::*;

        let uuid = format!("{}", uuid::Uuid::new_v4());
        
        let new_person = models::NewPerson {
            id: uuid.parse::<i32>().unwrap(),
            name: &msg.name,
            super_power: msg.super_power,
            rich: msg.rich,
            genius: msg.genius,
        };

        let conn: &PgConnection = &self.0.get().unwrap();

        diesel::insert_into(people)
            .values(&new_person)
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error inserting person"))?;
        
        let mut items = people
            .filter(id.eq(uuid.parse::<i32>().unwrap()))
            .load::<models::Person>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading person"))?;

        Ok(items.pop().unwrap())
    }
}

