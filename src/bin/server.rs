use chess_alpha_beta::alpha_beta::{get_best_move, ValueType};

use actix_web::{
    http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder,
};

#[derive(serde::Deserialize)]
struct GetBestMoveRequest {
    search_depth: std::num::NonZeroU32,
    fen: String,
}

#[derive(serde::Serialize)]
struct GetBestMoveResponse {
    m: String,
    value: ValueType,
}

#[post("/get_best_move")]
async fn api_get_best_move(json: web::Json<GetBestMoveRequest>) -> impl Responder {
    if json.search_depth.get() > 10 {
        return HttpResponse::BadRequest()
            .content_type(mime::TEXT_PLAIN_UTF_8)
            .body("search_depth is too large!");
    }

    let board = match pleco::Board::from_fen(&json.fen) {
        Ok(x) => x,
        Err(e) => {
            return HttpResponse::BadRequest()
                .content_type(mime::TEXT_PLAIN_UTF_8)
                .body(format!("Failed to parse FEN: {e:?}"));
        }
    };
    return match get_best_move(&board, json.search_depth) {
        Ok(ok) => HttpResponse::Ok().json(GetBestMoveResponse {
            m: ok.m.to_string(),
            value: ok.value,
        }),
        Err(e) => HttpResponse::BadRequest()
            .content_type(mime::TEXT_PLAIN_UTF_8)
            .body(format!("get_best_move failed: {e}")),
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info"),
    )
    .init();

    let server = HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(web::scope("/api").service(api_get_best_move))
            .route("/healthy", web::get().to(HttpResponse::Ok))
    })
    .bind(("localhost", 8080))?;
    log::info!("Server listening {:?}", server.addrs());
    server.run().await
}
