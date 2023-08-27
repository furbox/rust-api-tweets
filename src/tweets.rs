use actix_web::web::{Path, Data};
use actix_web::{get, post, HttpResponse};
use diesel::query_dsl::methods::{OrderDsl, LimitDsl};
use diesel::{PgConnection, ExpressionMethods};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Insertable, Queryable, RunQueryDsl};
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;
use crate::constants::APPLICATION_JSON;
use super::schema::tweets;

#[derive(Queryable, Insertable, Serialize)]
struct Tweet {
    id: Uuid,
    create_at: NaiveDateTime,
    message: String,
}
impl Tweet {
    fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            create_at : chrono::Local::now().naive_local(),
            message,
        }
    }
}
//obtener todos los tweets
#[get("/tweets")]
pub async fn get_tweets(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    use crate::schema::tweets::dsl::*;

    let mut conn = pool.get().expect("No se pudo obtener una conexion de la pool");
    
    let result = tweets.order(create_at.desc()).limit(10).load::<Tweet>(&mut conn);

    let response = match result {
        Ok(tws) => tws,
        Err(_) => vec![],
    };

    HttpResponse::Ok().content_type(APPLICATION_JSON).json(response)
}

//crear un tweet

#[post("/tweets")]
pub async fn create_tweet(req_body: String, pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    let nuevo_tweet = Tweet::new(req_body);
    let mut conn = pool.get().expect("No se pudo obtener una conexion de la pool");

    diesel::insert_into(tweets::table)
        .values(&nuevo_tweet)
        .execute(&mut conn)
        .expect("Error al insertar el tweet");

    HttpResponse::Created().content_type(APPLICATION_JSON).json(&nuevo_tweet)
}
//obtener un tweet
#[get("/tweets/{id}")]
pub async fn get_tweet(path: Path<(String,)>) -> HttpResponse {
    let tweet = format!("este es el tweet {:?}", path.0);
    HttpResponse::Ok().content_type(APPLICATION_JSON).json(tweet)
}