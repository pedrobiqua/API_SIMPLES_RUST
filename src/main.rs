use actix_web::*;

mod routes;
use routes::ping::{ping};
use routes::info::{info};
use routes::catalogo::{catalogo};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // Criando o servidor e suas rotas
    let api = HttpServer::new(|| {
        actix_web::App::new()
        .route("/", web::get().to(ping))
        .route("/info", web::get().to(info))
        .route("/catalogo", web::get().to(catalogo))
            
    });

    // Porta que o servidor vai rodar
    let porta = 8080;
    let api = api.bind(format!("127.0.0.1:{}", porta))
    .expect("Não conseguiu conectar na porta 8080");

    // Mensagem de sucesso
    println!(
        "Servidor conectado \n link: http://localhost:{}", porta
    );

    // Rodando o servidor
    return api.run().await;
}
