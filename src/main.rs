mod routes;
mod middlewares;

use routes::router;

pub fn main() {
    dotenv::dotenv().ok();

    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router()).unwrap();
}