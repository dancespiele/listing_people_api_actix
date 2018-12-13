//! people model
use actix_web::*;
use actix::prelude::*;
use diesel;
use diesel::prelude::*;
use db::DbExecutor;
use uuid;
use endpoints::people::structs::{GetPerson, AllPeople, DeletePerson, People};
use models;
use schema;
use error::ServiceError;

/// Message to create person
impl Message for People {
    type Result = Result<Vec<models::Person>, ServiceError>;
}

/// Message to getPerson
impl Message for GetPerson {
    type Result = Result<models::Person, ServiceError>;
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
impl Handler<People> for DbExecutor {
    type Result = Result<Vec<models::Person>, ServiceError>;

    fn handle(&mut self, messages: People, _: &mut Self::Context) -> Self::Result {
        use self::schema::people::dsl::*;

        println!("Body: {:#?}", messages);
        
        let conn: &PgConnection = &self.0.get().unwrap();

        let new_people = messages.list.iter()
            .map(|msg| {
                let uuid = format!("{}", uuid::Uuid::new_v4());
                models::NewPerson {
                    id: uuid.parse::<String>().expect("problem to pass to String from uuid format"),
                    name: &msg.name,
                    super_power: msg.super_power,
                    rich: msg.rich,
                    genius: msg.genius,
                }
            }).collect::<Vec<_>>(); 
        
        let items = diesel::insert_into(people)
                .values(new_people)
                .get_results(conn)
                .map_err(|error| ServiceError::InternalServerError(format!("{:#?}", error)))?;

        println!("Response: {:#?}", items);

        Ok(items)
    }
}

///Get one person
impl Handler<GetPerson> for DbExecutor {
    type Result = Result<models::Person, ServiceError>;

    fn handle(&mut self, msg: GetPerson, _: &mut Self::Context) -> Self::Result {
        use self::schema::people::dsl::*;

        let person_name = &msg.name;

        let conn: &PgConnection = &self.0.get().expect("Error to connect to database");

        let item = people
            .filter(name.eq(person_name))
            .first::<models::Person>(conn)
            .map_err(|error| {
                if error.to_string() == "NotFound" {
                    ServiceError::NotFound(format!("The person {} doesn't exist in the database", person_name))
                } else {
                    ServiceError::InternalServerError(format!("{:#?}", error))
                }
            })?;

        println!("Response: {:#?}", item);

        Ok(item)
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

        println!("Response: {:#?}", items);

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

        let item = match diesel::delete(people
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
        };

        println!("Response: {:#?}", item);

        item
    }
}
