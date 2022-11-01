use std::pin::Pin;

use futures::{FutureExt, TryFutureExt, future};
use gotham::handler::HandlerFuture;
use gotham::hyper::StatusCode;
use gotham::hyper::header::{HeaderMap, ORIGIN};
use gotham::middleware::Middleware;
use gotham::prelude::*;
use gotham::state::State;

use std::env;

#[derive(Clone, NewMiddleware)]
pub struct CorsMiddleware;

impl Middleware for CorsMiddleware {
    fn call<Chain>(self, state: State, chain: Chain) -> Pin<Box<HandlerFuture>>
        where
            Chain: FnOnce(State) -> Pin<Box<HandlerFuture>>,
        {
            let allowed_origins = env::var("WHITELIST").unwrap_or_else(|_| "*".to_string());

            let header_map = HeaderMap::borrow_from(&state);

            let requests_origin = match header_map.get(ORIGIN) {
                Some(origin) => origin.to_str().unwrap().to_string(),
                None => "".to_string(),
            };

            let response_allowed_origin = if allowed_origins.contains(',') {
                allowed_origins.split(',').find(|origin| origin.trim() == requests_origin).unwrap_or("").to_owned()
            } else {
                allowed_origins
            };

            let result = chain(state);

            let modified_result = result.and_then(move |(state, mut response)| {
                { 
                    response.headers_mut().insert(
                        "Access-Control-Allow-Origin",
                        response_allowed_origin.parse().unwrap(),
                    );
                    *response.status_mut() = StatusCode::UNAUTHORIZED;
                };
                future::ok((state, response))
            });

            modified_result.boxed()
        }
}
