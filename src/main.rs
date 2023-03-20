use actix_web::{web, App, HttpServer};

mod db;
mod handlers;
mod models;
mod schema;

#[macro_use]
extern crate diesel;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::create_pool(&database_url).expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/users").route(web::get().to(handlers::get_users)))
            .service(web::resource("/users").route(web::post().to(handlers::create_user)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
