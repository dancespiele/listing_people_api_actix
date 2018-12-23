use actix::prelude::*;
use actix_web::{http::Method, App, middleware, error, HttpResponse};
use db::{DbExecutor, AppState, GraphQLExecutor, GraphQLState, init_pool};
use endpoints::people::messages::SendMessage;
use middlewares::logger::LocalLogger;
use endpoints::graphql::{create_schema};

/// return all the resources and middlewares of the api
pub fn routes_db(db: Addr<DbExecutor>) -> App<AppState> {
    App::with_state(AppState{ db })
        .middleware(middleware::Logger::default())
        .middleware(LocalLogger)
        .resource("/people", |r| {
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

pub fn routes_graphql(url: String) -> App<GraphQLState> {
    let schema = std::sync::Arc::new(create_schema());

    let pool = init_pool(url);

    let addr = SyncArbiter::start(3, move || GraphQLExecutor::new(schema.clone(), pool.clone()));

    App::with_state(GraphQLState{ 
        executor: addr.clone(),
    })
        .middleware(middleware::Logger::default())
        .resource("/graphql", |r| r.method(Method::POST).with_async(SendMessage::graphql))
        .resource("/graphiql", |r| r.method(Method::GET).f(SendMessage::graphiql))
}