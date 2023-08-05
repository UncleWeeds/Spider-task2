use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use env_logger::Env;

// To actually use them
mod services {
    pub mod db;
    pub mod endpoints;
}
mod models {
    pub mod models;
}
pub mod schema;
use crate::services::endpoints::{
    create_new_user, get_all_present_user, get_some_user, update_particular_user, delete_particular_user, login_user,
};
use crate::services::db::get_connection_pool;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_connection_pool();
    println!("Connection to DB Established !\n");

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive()) // Permissive CORS setup
            .app_data(web::Data::new(pool.clone()))
            .service(create_new_user)
            .service(get_all_present_user)
            .service(get_some_user)
            .service(update_particular_user)
            .service(delete_particular_user)
            .service(login_user)
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}


