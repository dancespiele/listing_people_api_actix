use actix_web::{HttpRequest, Result,
    middleware::{Middleware, Started}};

pub struct LocalLogger;

///middleware for log all the events from te server
impl<S> Middleware<S> for LocalLogger {
    ///log all the request
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        println!("Request: {:#?}", req.request());
        println!("Query parameters: {:#?}", req.query());
        Ok(Started::Done)
    }
}
