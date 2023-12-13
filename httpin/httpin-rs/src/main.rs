#![deny(clippy::unwrap_used)]
//#![forbid(unsafe_code)]

use axum::{
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::any,
    Router,
};
use clap::Parser;
use http_body_util::BodyExt;
use std::error::Error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn handler(request: Request) -> Result<String, Response> {
    let first_line = format!("{} {}", request.method(), request.uri());

    let headers: String = request
        .headers()
        .iter()
        .map(|(name, value)| format!("{}: {:}", name, value.to_str().unwrap_or_default()))
        .collect::<Vec<String>>()
        .join("\n");

    let (_parts, body) = request.into_parts();

    // this wont work if the body is an long running stream
    let bytes = body
        .collect()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?
        .to_bytes();

    let request_body = unsafe { String::from_utf8_unchecked(bytes.to_vec()) };

    let html = format!("{first_line}\n\n{headers}\n\n{request_body}");

    tracing::debug!(body = ?html);

    Ok(html)
}

#[derive(Debug, Parser)]
struct Args {
    ///host
    #[arg(short = 'H', long)]
    host: Option<String>,

    #[arg(short, long)]
    ///port
    port: Option<u16>,
}

impl Args {
    fn address(&self) -> String {
        format!("{}:{}", self.host.as_deref().unwrap_or("0.0.0.0"), self.port.unwrap_or(3000))
    }

    fn as_url(&self) -> String {
        format!("http://localhost:{}", self.port.unwrap_or(3000))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "httpin_rs=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();

    let app = Router::new().route("/", any(handler)).route("/*all", any(handler));

    tracing::info!("listen on {}, {}", args.address(), args.as_url());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(args.address()).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
