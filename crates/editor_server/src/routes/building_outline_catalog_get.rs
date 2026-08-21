//! `GET /api/建物外形カタログ`: 起動時に検証した版付きカタログを返す。

use axum::{Json, extract::State};

use crate::{resource::建物外形カタログ, server_state::サーバー状態};

pub async fn 建物外形カタログを返す(State(状態): State<サーバー状態>) -> Json<建物外形カタログ> {
    Json(状態.建物外形カタログ().clone())
}
