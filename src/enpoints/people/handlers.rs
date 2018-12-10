//! people model
use actix_web::*;
use actix::prelude::*;
use diesel;
use diesel::prelude::*;
use db::DbExecutor;
use uuid;
use enpoints::people::structs::{CreatePerson, GetPerson, AllPeople, DeletePerson};
use models;
use schema;
use error::ServiceError;

/// Message to create person
impl Message for CreatePerson {
    type Result = Result<models::Person, ServiceError>;
}

/// Message to getPerson
impl Message for GetPerson {
    type Result = Result<Vec<models::Person>, ServiceError>;
}

/// Message to get all people
impl Message for AllPeople {
    type Result = Result<Vec<models::Person>, ServiceError>;
}

/// Message to delete Person
impl Message for DeletePerson {
    type Result = Result<String, ServiceError>;
}

/// Implement Actor
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

///save the person in the database
impl Handler<CreatePerson> for DbExecutor {
    type Result = Result<models::Person, ServiceError>;

    fn handle(&mut self, msg: CreatePerson, _: &mut Self::Context) -> Self::Result {
        use self::schema::people::dsl::*;

        let uuid = format!("{}", uuid::Uuid::new_v4());

        println!("Body: {:#?}", &msg);
        
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
            .map_err(|error| ServiceError::InternalServerError(format!("{:#?}", error)))?;
        
        let mut items = people
            .filter(id.eq(uuid.parse::<String>().unwrap()))
            .load::<models::Person>(conn)
            .map_err(|error| ServiceError::InternalServerError(format!("{:#?}", error)))?;

        Ok(items.pop().unwrap())
    }
}

///Get one person
impl Handler<GetPerson> for DbExecutor {
    type Result = Result<Vec<models::Person>, ServiceError>;

    fn handle(&mut self, msg: GetPerson, _: &mut Self::Context) -> Self::Result {
        use self::schema::people::dsl::*;

        let person_name = &msg.name;

        let conn: &PgConnection = &self.0.get().expect("Error to connect to database");

        match people
            .filter(name.eq(person_name))
            .load::<models::Person>(conn) {
                Ok(items) => {
                    if !items.is_empty() {
                        Ok(items)
                    } else {
                        Err(ServiceError::NotFound(format!("{} not found in database", person_name)))
                    }
                },
                Err(error) => Err(ServiceError::InternalServerError(format!("{:#?}", error)))
            }
    }
}

///get all the people
impl Handler<AllPeople> for DbExecutor {
    type Result = Result<Vec<models::Person>, ServiceError>;

    fn handle(&mut self, _: AllPeople, _: &mut Self::Context) -> Self::Result {
        use self::schema::people::dsl::*;

        let conn: &PgConnection = &self.0.get().expect("Error to connect to database");
        
        let items = people
            .order(schema::people::name.asc())
            .load::<models::Person>(conn)
            .map_err(|error| ServiceError::InternalServerError(format!("{:#?}", error)))?;

        Ok(items)
    }
}

///delete person
impl Handler<DeletePerson> for DbExecutor {
    type Result = Result<String, ServiceError>;

    fn handle(&mut self, msg: DeletePerson, _: &mut Self::Context) -> Self::Result {
        use self::schema::people::dsl::*;

        let person_name = &msg.name;

        let conn: &PgConnection = &self.0.get().expect("Error to connect to database");

        match diesel::delete(people
            .filter(name.eq(person_name)))
            .execute(conn) {
                Ok(items) => {
                    if items > 0 {
                        Ok(format!("{} was deleted successfully from the database", person_name))
                    } else {
                        Err(ServiceError::NotFound(format!("{} not found in database", person_name)))
                    }
                },
                Err(error) => Err(ServiceError::InternalServerError(format!("{:#?}", error)))
            }
    }
}
