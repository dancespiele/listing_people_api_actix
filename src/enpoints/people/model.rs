//! people model
//! use actix::prelude::*;
use actix_web::*;
use actix::prelude::*;
use diesel;
use diesel::prelude::*;
use db::DbExecutor;
use uuid;
use enpoints::people::structs::{CreatePerson, GetPerson};
use models;
use schema;

/// Message to create person
impl Message for CreatePerson {
    type Result = Result<models::Person, Error>;
}

/// Message to getPerson
impl Message for GetPerson {
    type Result = Result<models::Person, Error>;
}

/// Implement Actor
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

///save the person in the database
impl Handler<CreatePerson> for DbExecutor {
    type Result = Result<models::Person, Error>;

    fn handle(&mut self, msg: CreatePerson, _: &mut Self::Context) -> Self::Result {
        use self::schema::people::dsl::*;

        let uuid = format!("{}", uuid::Uuid::new_v4());
        
        let new_person = models::NewPerson {
            id: uuid.parse::<String>().expect("problem to pass to String from uuid format"),
            name: &msg.name,
            super_power: msg.super_power,
            rich: msg.rich,
            genius: msg.genius,
        };

        let conn: &PgConnection = &self.0.get().unwrap();

        diesel::insert_into(people)
            .values(&new_person)
            .execute(conn)
            .unwrap();
        
        let mut items = people
            .filter(id.eq(uuid.parse::<String>().unwrap()))
            .load::<models::Person>(conn)
            .expect("Error loading person");

        Ok(items.pop().unwrap())
    }
}

impl Handler<GetPerson> for DbExecutor {
    type Result = Result<models::Person, Error>;

    fn handle(&mut self, msg: GetPerson, _: &mut Self::Context) -> Self::Result {
        use self::schema::people::dsl::*;

        let person_name = &msg.name;

        let conn: &PgConnection = &self.0.get().expect("Error to connect to database");

        let mut items = people
            .filter(name.eq(person_name))
            .load::<models::Person>(conn)
            .expect("Person is not in the database");
        
        Ok(items.pop().unwrap())
    }
}