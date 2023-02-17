mod db;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use db::db_connection;
use mysql::prelude::*;

async fn create_user(req_body: String) -> impl Responder{
    HttpResponse::Ok().body("Hello user")
}

async fn auth_user(req_body: String) -> impl Responder{
    HttpResponse::Ok().body("Hello authenticated user")
}

async fn new_post() -> impl Responder{
    HttpResponse::Ok().body("new post")
}

async fn get_posts() -> impl Responder{
    HttpResponse::Ok().body("hey posts")
}

async fn delete_post() -> impl Responder{
    HttpResponse::Ok().body("deleted post")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut connection = db_connection();

    connection.query_drop(
        r"CREATE TABLE payment (
            customer_id int not null,
            amount int not null,
            account_name text
        )").expect("an error occured");

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/user")
                    .route("/create", web::post().to(create_user))
                    .route("/auth", web::get().to(auth_user))
            )
            .service(web::scope("/posts")
                .route("/new", web::post().to(new_post))
                .route("/all", web::get().to(get_posts))
                .route("/delete", web::delete().to(delete_post))            
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
