use axum::Json;
use serde_json::{json, Value};

pub async fn health_handler() -> Json<Value> {
    let (jit_avail, jit_en, jit_reason) = erock::health_fields();
    Json(json!({
        "status": "ok",
        "jit_available": jit_avail,
        "jit_enabled": jit_en,
        "jit_reason": jit_reason
    }))
}
