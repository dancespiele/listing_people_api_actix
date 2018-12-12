use actix_web::{AsyncResponder, HttpResponse, State,
    FutureResponse, Json, Path, ResponseError};
use endpoints::people::structs::{GetPerson, AllPeople, DeletePerson, People};
use db::AppState;
use futures::Future;

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
}
