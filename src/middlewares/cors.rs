use std::pin::Pin;

use futures::{FutureExt, TryFutureExt, future};
use gotham::handler::HandlerFuture;
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
        dotenv::dotenv().ok();

        let allowed_origins = env::var("WHITELIST").unwrap_or_else(|_| "*".to_string());
        
        let header_map = HeaderMap::borrow_from(&state);

        let origin_request = match header_map.get(ORIGIN) {
            Some(origin) => origin.to_str().unwrap().to_string(),
            None => "".to_string(),
        };

        let origin_response = if allowed_origins.contains(",") {
            allowed_origins.split(',').find(|origin| origin == &origin_request).unwrap_or("").to_owned()
        } else {
            allowed_origins
        };

        let result = chain(state);

        let modified_result = result.and_then(move |(state, mut response)| {
            {
                let headers = response.headers_mut();

                headers.insert(
                    "Access-Control-Allow-Origin",
                    format!(
                        "{}",
                        origin_response,
                    )
                    .parse()
                    .unwrap(),
                );
            };
            future::ok((state, response))
        });

        modified_result.boxed()
    }
}
