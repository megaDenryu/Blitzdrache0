//! `GET /api/建物外形カタログ`: いま有効な版付きカタログを返す。格子を保存するたびに組み直された値へ差し替わる。

use axum::{Json, extract::State, response::IntoResponse, response::Response};

use crate::server_state::サーバー状態;

pub async fn 建物外形カタログを返す(State(状態): State<サーバー状態>) -> Response {
    match 状態.建物外形カタログの写しを取る() {
        Ok(カタログ) => Json(カタログ).into_response(),
        Err(応答) => *応答,
    }
}
