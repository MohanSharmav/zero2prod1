use std::net::TcpListener;
use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::dev::Server;
use sqlx::{PgConnection, Pool, Postgres};

pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;


async fn health_check()->HttpResponse
{
    HttpResponse::Ok().finish()
}
//
// async fn subscribe()-> HttpResponse{
//     HttpResponse::Ok().finish()
// }

#[derive(serde::Deserialize)]
struct  FormData{
email: String,
    name: String
}


async fn subscribe(_form:web::Form<FormData>)-> HttpResponse{
    HttpResponse::Ok().finish()
}

pub  fn run(listener:TcpListener, connection: Pool<Postgres>,) -> Result<Server, std::io::Error>
{
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::get().to(subscribe))
    })
        .listen(listener)?
        .run();

    // let server = Server::new(handle: val, fut: val);
    Ok(server)
}

