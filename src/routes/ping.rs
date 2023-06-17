use actix_web::*;

pub async fn ping() -> HttpResponse  {
    return HttpResponse::Ok().body("Concetado");
}