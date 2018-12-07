use actix::prelude::*;
use actix_web::{http::Method, App, middleware, error, HttpResponse};
use db::{DbExecutor, AppState};
use enpoints::people::messages::SendMessage;

/// return all the resources and middlewares of the api
pub fn routes(db: Addr<DbExecutor>) -> App<AppState> {
    App::with_state(AppState{ db })
        .middleware(middleware::Logger::default())
        .resource("/person", |r| {
            r.method(Method::POST)
                .with_async_config(SendMessage::send_create, |cfg| {
                    cfg.0.limit(4096)
                        .error_handler(|err, _req| {
                            let description = format!("{}", err);
                            error::InternalError::from_response(
                                err, HttpResponse::BadRequest().json(description)).into()
                        });
                });
            r.method(Method::GET).with_async(SendMessage::send_get_all);
        })
        .resource("/person/{name}", |r| {
            r.method(Method::GET).with_async(SendMessage::send_get_person);
            r.method(Method::DELETE).with_async(SendMessage::send_delete);
        })
}