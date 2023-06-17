use actix_web::*;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

pub async fn info() -> HttpResponse  {
    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
            "Versão: {} <br> Autor: {} <br> Nome app: {}", VERSION, AUTHORS, NAME
        )
    );
}