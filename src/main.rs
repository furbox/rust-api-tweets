use actix_web::{App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
//mod
mod tweets;
mod likes;
mod constants;
mod schema;

#[actix_web::main] // or #[tokio::main]
async fn main()  -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool.");
    HttpServer::new(move || {
        App::new().app_data(pool.clone()).service(tweets::get_tweets).service(tweets::create_tweet).service(tweets::get_tweet).service(likes::get_likes).service(likes::remove_like).service(likes::create_like)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
