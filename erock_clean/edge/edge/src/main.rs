use axum::{routing::{post, get}, Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio;

use erock::eval;  # Import fixes unresolved [so-8]

#[derive(Deserialize)]
struct EvalRequest {
    expr: String,
    variables: HashMap<String, Vec<f64>>,
}

#[derive(Serialize)]
struct EvalResponse {
    result: f64,
    error: Option<String>,
}

async fn evaluate(Json(req): Json<EvalRequest>) -> Json<EvalResponse> {
    let mut vars = req.variables;
    match eval(&req.expr, &mut vars) {
        Ok(r) => Json(EvalResponse { result: r, error: None }),
        Err(e) => Json(EvalResponse { result: 0.0, error: Some(e) }),
    }
}

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/evaluate", post(evaluate))
        .route("/health", get(health));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("eRock server listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
