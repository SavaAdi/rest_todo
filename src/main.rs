#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_service::Service;
use actix_web::{App, HttpServer, HttpResponse};
use futures::future::{ok, Either};
use log;
use env_logger;
mod database;
mod json_serialization;
mod models;
mod processes;
mod schema;
mod state;
mod to_do;
mod views;
mod auth;

// RUST_LOG="info,parser::expression=info,actix_web=info"
// cargo run

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                // srv => routing
                // req => service request
                let request_url: String = String::from(req.uri().path().clone());

                let passed: bool;

                if *&req.path().contains("/item/") {
                    match auth::process_token(&req) {
                        Ok(_token) => {
                            passed = true;
                        },
                        Err(_message) => {
                            passed = false;
                        }
                    }
                }
                else {
                    passed = true;
                }
               
                let end_result = match passed {
                    true => {
                        Either::Left(srv.call(req))
                    },
                    false => {
                        Either::Right(
                            ok(req.into_response(
                                HttpResponse::Unauthorized()
                                    .finish()
                                    .into_body()))
                        )
                    }
                };

                // move keyword moves the ownership of request_url here
                async move {
                    let result = end_result.await?;
                    log::info!("{} -> {}", request_url, &result.status());                 
                    Ok(result)
                }
            }).configure(views::views_factory);

        return app;
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await    
}
