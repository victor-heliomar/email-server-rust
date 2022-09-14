pub mod controllers;

use gotham::prelude::*;
use gotham::router::{build_simple_router, Router};

pub fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(controllers::say_hello);
        route.get("/send_mail").with_query_string_extractor::<controllers::SendMailQueryStringExtractor>().to(controllers::send_mail);       
    })
}