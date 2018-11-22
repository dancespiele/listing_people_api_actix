use actix_web::{AsyncResponder, HttpResponse, HttpRequest,
    FutureResponse, Json};
use enpoints::people::model::CreatePerson;
use futures::{Future};
use db::AppState;

/// Async request handler
pub fn insert_person((person, req): (Json<CreatePerson>, HttpRequest<AppState>))
    -> FutureResponse<HttpResponse> {
    req.state()
        .db
        .send(person.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(person) => Ok(HttpResponse::Ok().json(person)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}