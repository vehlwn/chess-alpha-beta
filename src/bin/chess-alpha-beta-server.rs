use actix_cors::Cors;
use actix_web::{http::header, post, web, App, HttpResponse, HttpServer, Responder};

use chess_alpha_beta::alpha_beta::{get_best_move, ValueType};

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

/// Server program for chess-alpha-beta
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// TCP port where to bind HTTP server
    #[arg(short, long, default_value = "8081")]
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info"),
    )
    .init();
    use clap::Parser;
    let args = Args::parse();

    let server = HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(
                // default settings are overly restrictive to reduce chance of
                // misconfiguration leading to security concerns
                Cors::default()
                    // allow any port on localhost
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost")
                    })
                    // set allowed methods list
                    .allowed_methods(vec!["GET", "POST"])
                    // set allowed request header list
                    .allowed_headers(&[header::AUTHORIZATION, header::ACCEPT])
                    // add header to allowed list
                    .allowed_header(header::CONTENT_TYPE)
                    // set list of headers that are safe to expose
                    .expose_headers(&[header::CONTENT_DISPOSITION])
                    // allow cURL/HTTPie from working without providing Origin headers
                    .block_on_origin_mismatch(false)
                    // set preflight cache TTL
                    .max_age(3600),
            )
            .service(web::scope("/api").service(api_get_best_move))
            .route("/healthy", web::get().to(HttpResponse::Ok))
    })
    .bind(("0.0.0.0", args.port))?;
    log::info!("Server listening {:?}", server.addrs());
    server.run().await
}