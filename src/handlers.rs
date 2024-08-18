use crate::error::{ServerError, ServerResult};
use anyhow::{anyhow, Context};
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::Response,
    Json,
};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row};
use std::{collections::HashMap, time::SystemTime};
use tokio::sync::OnceCell;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

/// Build-in static files
static STATIC_FILES: OnceCell<HashMap<&'static str, static_files::Resource>> =
    OnceCell::const_new();

/// Get static resource
pub async fn get_static_resource(path: Path<String>) -> ServerResult<Response<Body>> {
    let path = path.0;
    let mut static_files = STATIC_FILES
        .get_or_init(|| async { generate() })
        .await
        .iter();
    match static_files.find(|(k, _)| k.contains(&path)) {
        Some((_, v)) => {
            let mime_type = if v.mime_type.eq("application/octet-stream") {
                "application/text"
            } else {
                v.mime_type
            };
            create_response_with_data(StatusCode::OK, mime_type, v.data)
        }
        None => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())?),
    }
}

fn create_response_with_data(
    status: StatusCode,
    content_type: &str,
    data: impl Into<Body>,
) -> ServerResult<Response<Body>> {
    Ok(Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, content_type)
        .body(data.into())?)
}

#[derive(Deserialize)]
pub struct RequestBody {
    text: String,
}

#[derive(Deserialize)]
pub struct RequestQuery {
    id: String,
}

pub async fn upload_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<RequestBody>,
) -> ServerResult<Json<Value>> {
    let text = payload.text;

    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    let sha256_hash = format!("{:x}", hasher.finalize());

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .context("Time went backwards")?
        .as_nanos();
    let timestamp_hex = format!("{:x}", timestamp);

    let id = format!("{}{}", sha256_hash, timestamp_hex);

    let _ = sqlx::query("INSERT INTO tb_app (id, md) VALUES ($1, $2)")
        .bind(&id)
        .bind(&text)
        .execute(&pool)
        .await?;

    Ok(Json(serde_json::json!({
        "url": format!("/index.html?id={}", id)
    })))
}

pub async fn get_handler(
    State(pool): State<PgPool>,
    Query(query): Query<RequestQuery>,
) -> ServerResult<Json<Value>> {
    let result = sqlx::query("SELECT md FROM tb_app WHERE id = $1")
        .bind(query.id)
        .fetch_optional(&pool)
        .await?;

    match result {
        Some(record) => {
            let md: String = record.get(0);
            let text = html_escape::decode_html_entities(&md).to_string();
            Ok(Json(serde_json::json!({ "text": text })))
        }
        None => Err(ServerError::BadRequest(anyhow!(
            "No data found for the given ID"
        ))),
    }
}
