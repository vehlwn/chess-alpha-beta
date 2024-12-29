use anyhow::Context;

use axum::extract::Json;
use axum::http::header;
use axum::http::{Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use tower_http::cors::{AllowOrigin, CorsLayer};

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

async fn api_get_best_move(Json(json): Json<GetBestMoveRequest>) -> Response {
    if json.search_depth.get() > 10 {
        return (StatusCode::BAD_REQUEST, "search_depth is too large!")
            .into_response();
    }

    let board = match pleco::Board::from_fen(&json.fen) {
        Ok(x) => x,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Failed to parse FEN: {e:?}"),
            )
                .into_response();
        }
    };
    return match get_best_move(&board, json.search_depth) {
        Ok(ok) => axum::response::Json(GetBestMoveResponse {
            m: ok.m.to_string(),
            value: ok.value,
        })
        .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            format!("get_best_move failed: {e}"),
        )
            .into_response(),
    };
}

/// Server program for chess-alpha-beta
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Host and port where to bind HTTP server
    #[arg(short, long, default_value = "127.0.0.1:8081")]
    bind_addr: std::net::SocketAddr,

    /// Log verbosity
    #[arg(short, long, default_value = "info")]
    log_level: log::LevelFilter,

    /// Use systemd_journal_logger instead of env_logger
    #[arg(short, long)]
    journald: bool,
}

fn init_logging(args: &Args) -> anyhow::Result<()> {
    if args.journald {
        systemd_journal_logger::JournalLog::new()
            .context("Failed to crate journal log")?
            .install()
            .context("Failed to install journal log")?;
        log::set_max_level(args.log_level);
    } else {
        let mut builder = env_logger::Builder::from_default_env();
        builder.filter_level(args.log_level).init();
    }
    return Ok(());
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use clap::Parser;
    let args = Args::parse();
    init_logging(&args).context("Failed to init logging")?;

    let app = axum::Router::new()
        .route("/api/get_best_move", post(api_get_best_move))
        .route("/healthy", get(|| std::future::ready("ok")))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(|origin, _request_parts| {
                    origin.as_bytes().starts_with(b"http://localhost")
                }))
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([header::CONTENT_TYPE]),
        );

    let listener = tokio::net::TcpListener::bind(&args.bind_addr)
        .await
        .with_context(|| {
            format!("Failed to bind HTTP server to '{}'", args.bind_addr)
        })?;
    log::info!("Server listening {:?}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;
    return Ok(());
}
