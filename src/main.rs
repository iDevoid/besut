use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::io::Result;
use std::env;

mod report;

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mut server = HttpServer::new(|| App::new().configure(report::routes));
    
    server = match ListenFd::from_env().take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}