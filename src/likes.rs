use actix_web::web::Path;
use actix_web::{get, post, delete, HttpResponse};
use crate::constants::APPLICATION_JSON;

//obtener likes de un tweet
#[get("/tweets/{id}/likes")]
pub async fn get_likes(path: Path<(String,)>) -> HttpResponse {
    let likes = 100;
    let tweet = format!("tweet {:?} links {:?}", path.0, likes);
    HttpResponse::Ok().content_type(APPLICATION_JSON).json(tweet)
}

//crear un like
#[post("/tweets/{id}/likes")]
pub async fn create_like(path: Path<(String,)>) -> HttpResponse {
    let like = "ok";
    let tweet = format!("tweet {:?} create like {:?}", path.0, like);
    HttpResponse::Created().content_type(APPLICATION_JSON).json(tweet)
}

//remover like
#[delete("/tweets/{id}/likes")]
pub async fn remove_like(path: Path<(String,)>) -> HttpResponse {
    let like = "ok";
    let tweet = format!("tweet {:?} delete like {:?}", path.0, like);
    HttpResponse::Ok().content_type(APPLICATION_JSON).json(tweet)
}
