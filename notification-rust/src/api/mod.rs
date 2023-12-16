use std::error::Error;

use http::{Request, Response};

use crate::service::HelloWorldService;

pub trait HttpHandler {
    fn handle<Body>(&self, req: &mut Request<Body>) -> Result<Response<Body>, Box<dyn Error>>;
}

pub struct HelloWorldController {
    pub service: HelloWorldService,
}

impl HelloWorldController {
    pub fn new(service: HelloWorldService) -> Self {
        Self { service }
    }

    pub fn handle(&self, req: Request<()>) -> Result<Response<&str>, Box<dyn Error>> {
        let mut response = Response::new("Hello World!");
        *response.status_mut() = http::StatusCode::OK;
        Ok(response)
    }
}