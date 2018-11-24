use actix_web::{AsyncResponder, HttpResponse, HttpRequest,
    FutureResponse, Json, Path};
use enpoints::people::structs::{CreatePerson, GetPerson};
use futures::{Future};
use db::AppState;

/// Route to create person
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

/// Route to create person
pub fn get_person((person, req): (Path<GetPerson>, HttpRequest<AppState>))
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