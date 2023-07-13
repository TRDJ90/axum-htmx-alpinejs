mod templates;

use templates::{ChartTemplate, IndexTemplate};

use askama::{helpers::TemplateLoop, Template};
use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber::FmtSubscriber;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    let body = template.render().unwrap();
    (StatusCode::OK, Html(body))
}

async fn hello() -> impl IntoResponse {
    let template = HelloTemplate { name: "world" };
    let reply = template.render().unwrap();
    (StatusCode::OK, Html(reply))
}

async fn chart() -> impl IntoResponse {
    let template = ChartTemplate {};
    let body = template.render().unwrap();
    (StatusCode::OK, Html(body))
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    let trace_sub = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(trace_sub).unwrap();

    let app = Router::new()
        .route("/", get(index))
        .route("/hello", get(hello))
        .route("/chart", get(chart));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3300));

    tracing::info!("Listining on {}", listener.local_addr().unwrap());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
