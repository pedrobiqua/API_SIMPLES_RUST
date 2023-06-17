use actix_web::*;

pub async fn catalogo() -> HttpResponse  {
    return HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body("{ \"mensagem\":\"olá\"}")
    ;
}