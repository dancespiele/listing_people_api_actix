extern crate actix_web;
use listing_people_api_actix::enpoints;
use actix_web::{HttpRequest, HttpResponse, test::TestServer};
use enpoints::people::messages::{SendMessage};

#[test]
fn get_person() {
    let mut srv = TestServer::new(|app| app.handler(SendMessage::send_get_person));

    let request = srv.get().finish().unwrap();

    let response = srv.execute(request.send()).unwrap();
    assert!(response.status().is_success());
}