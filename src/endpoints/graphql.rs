use juniper::{FieldResult, RootNode};
use diesel::prelude::*;
use db::Conn;
use error::ServiceError;
use models::{Person, NewPerson};
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
    field find_person(&executor, person_name: String) -> FieldResult<Vec<Person>> {
        use schema::people::dsl::*;

        let conn: &PgConnection = &executor.context().connection;

        let items = people
            .filter(name.eq(person_name))
            .load::<Person>(conn)
            .map_err(|error| ServiceError::InternalServerError(format!("{:#?}", error)))?;
        
        Ok(items)
    }

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
    field createPerson(&executor, new_person: NewPerson) -> FieldResult<Vec<Person>> {
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
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
