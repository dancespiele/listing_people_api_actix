use juniper::{FieldResult, RootNode};
use diesel::prelude::*;
use db::Conn;
use error::ServiceError;
use models::{Person, NewPersonGraph, NewPerson};
use uuid;
use schema;

pub struct Context {
    pub connection: Conn,
}

impl Context {
    pub fn new(conn: Conn) -> Context {
        Context {connection: conn}
    }
}

impl juniper::Context for Context {}

pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
    description: "Queries people"

    /// Find a person with the name given
    field find_person(&executor, person_name: String) -> FieldResult<Person> {
        use schema::people::dsl::*;

        let conn: &PgConnection = &executor.context().connection;

        let item = people
            .filter(name.eq(&person_name))
            .first::<Person>(conn)
            .map_err(|error| {
                if error.to_string() == "NotFound" {
                    ServiceError::NotFound(format!("The person {} doesn't exist in the database", &person_name))
                } else {
                    ServiceError::InternalServerError(format!("{:#?}", error))
                }
            })?;

        Ok(item)
    }

    /// Show all the people saved in the database
    field people(&executor) -> FieldResult<Vec<Person>> {
        use schema::people::dsl::*;

        let conn: &PgConnection = &executor.context().connection;

        let items = people
            .order(schema::people::name.asc())
            .load::<Person>(conn)
            .map_err(|error| ServiceError::InternalServerError(format!("{:#?}", error)))?;
        
        Ok(items)
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Context |&self| {
    description: "Mutations people"

    /// create a person with his or her skills
    field create_person(&executor, new_person: NewPersonGraph) -> FieldResult<Vec<Person>> {
        use schema::people::dsl::*;

        let conn: &PgConnection = &executor.context().connection;
        let uuid = format!("{}", uuid::Uuid::new_v4());

        let person = NewPerson {
            id: uuid.parse::<String>().expect("problem to pass to String from uuid format"),
            name: new_person.name,
            super_power: new_person.super_power,
            rich: new_person.rich,
            genius: new_person.genius,
        };

        let item = diesel::insert_into(people)
            .values(person)
            .get_results(conn)
            .map_err(|error| ServiceError::InternalServerError(format!("{:#?}", error)))?;

        Ok(item)
    }

    /// Delete a person
    field delete_person(&executor, person_name: String) -> FieldResult<String> {
        use schema::people::dsl::*;

        let conn: &PgConnection = &executor.context().connection;

        let item = match diesel::delete(people
            .filter(name.eq(&person_name)))
            .execute(conn) {
                Ok(items) => {
                    if items > 0 {
                        Ok(format!("{} was deleted successfully from the database", &person_name))
                    } else {
                        Err(ServiceError::NotFound(format!("{} not found in database", &person_name)))?
                    }
                },
                Err(error) => Err(ServiceError::InternalServerError(format!("{:#?}", error)))?
        };

        item
    }

    /// Update the skills of the person
    field update_person(&executor, person: NewPersonGraph) -> FieldResult<Person> {
        use schema::people::dsl::*;

        let conn: &PgConnection = &executor.context().connection;

        let new_item = match diesel::update(people.filter(name.eq(person.name)))
            .set((rich.eq(person.rich), genius.eq(person.genius), super_power.eq(person.super_power)))
            .get_result(conn) {
                Ok(item) => item,
                Err(error) => Err(ServiceError::InternalServerError(format!("{:#?}", error)))?
            };

        Ok(new_item)
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
