#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use axum::{
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::any,
    Router,
};
use std::borrow::Cow;
use std::error::Error;

use http_body_util::BodyExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use httpin_rs::Args;

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

    let request_body = match String::from_utf8_lossy(&bytes) {
        Cow::Borrowed(s) => s.to_string(),
        Cow::Owned(s) => s, //hex::encode(bytes),
    };

    let text = format!("{first_line}\n\n{headers}\n\n{request_body}");

    tracing::debug!(body = ?text);

    Ok(text)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "httpin_rs=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::from_parse();

    let app = Router::new().route("/", any(handler)).route("/*all", any(handler));

    tracing::info!("listen on {}, {}", args.address(), args.as_url());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(args.address()).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
