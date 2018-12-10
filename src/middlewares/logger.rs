use actix_web::{HttpRequest, HttpResponse, Result,
    middleware::{Middleware, Started, Response}};

pub struct LocalLogger;

///middleware for log all the events from te server
impl<S> Middleware<S> for LocalLogger {
    ///log all the request
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        println!("Request: {:#?}", req.request());
        println!("Query parameters: {:#?}", req.query());
        Ok(Started::Done)
    }
    
    ///log all the response from the server
    fn response(&self, _req: &HttpRequest<S>, resp: HttpResponse)
        -> Result<Response>
    {
        println!("{:#?}", resp.body());
        Ok(Response::Done(resp))
    }
}
