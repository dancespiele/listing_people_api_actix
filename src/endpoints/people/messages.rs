use actix_web::{AsyncResponder, HttpResponse, HttpRequest, Error, State,
    FutureResponse, Json, Path, ResponseError};
use endpoints::people::structs::{GetPerson, AllPeople, DeletePerson, People, GraphQLData};
use db::{AppState, GraphQLState};
use futures::Future;
use juniper::http::graphiql::graphiql_source;

pub struct SendMessage;

///Struct to send messages to handler request
impl SendMessage {
    /// Receive json from argument and send to handler
    pub fn send_create(people: Json<People>, state: State<AppState>)
        -> FutureResponse<HttpResponse> {
        state
            .db
            .send(people.into_inner())
            .from_err()
            .and_then(|res| match res {
                Ok(new_people) => Ok(HttpResponse::Ok().json(new_people)),
                Err(error) => Ok(error.error_response()),
            })
            .responder()
    }

    /// Receive name parameter of url from argument and send to handler to get the person
    pub fn send_get_person((person, state): (Path<GetPerson>, State<AppState>))
        -> FutureResponse<HttpResponse> {
        state
            .db
            .send(person.into_inner())
            .from_err()
            .and_then(|res| match res {
                Ok(person) => Ok(HttpResponse::Ok().json(person)),
                Err(error) => Ok(error.error_response()),
            })
            .responder()
    }

    /// Send message to get all the people
    pub fn send_get_all(state: State<AppState>) -> FutureResponse<HttpResponse> {
        state
            .db
            .send(AllPeople)
            .from_err()
            .and_then(|res| match res {
                Ok(people) => Ok(HttpResponse::Ok().json(people)),
                Err(error) => Ok(error.error_response()),
            })
            .responder()
    }

    /// Send message to delete a person
    pub fn send_delete((person, state): (Path<DeletePerson>, State<AppState>))
        -> FutureResponse<HttpResponse> {
        state
            .db
            .send(person.into_inner())
            .from_err()
            .and_then(|res| match res {
                Ok(people) => Ok(HttpResponse::Ok().json(people)),
                Err(error) => Ok(error.error_response()),
            })
            .responder()
    }

    /// start graphql
    pub fn graphiql(_req: &HttpRequest<GraphQLState>) -> Result<HttpResponse, Error> {
        let html = graphiql_source("http://127.0.0.1:8088/graphql");
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html))
    }

    /// send message to graphql
    pub fn graphql(state: State<GraphQLState>, data: Json<GraphQLData>) 
        -> FutureResponse<HttpResponse> {
            state
                .executor
                .send(data.0)
                .from_err()
                .and_then(|res| match res {
                    Ok(people) => Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(people)),
                    Err(_) => Ok(HttpResponse::InternalServerError().into()),
                })
                .responder()
        } 
}
