pub mod controllers;

use gotham::prelude::*;
use gotham::pipeline::{new_pipeline, single_pipeline};
use gotham::router::{Router, build_router};

use crate::middlewares::cors::CorsMiddleware;

pub fn router() -> Router {
    let (chain, pipelines) = single_pipeline(new_pipeline().add(CorsMiddleware).build());
    
    build_router(chain, pipelines, |route| {
        route.get("/").to(controllers::say_hello);
        route.get("/send_mail").with_query_string_extractor::<controllers::SendMailQueryStringExtractor>().to(controllers::send_mail);       
    })
}