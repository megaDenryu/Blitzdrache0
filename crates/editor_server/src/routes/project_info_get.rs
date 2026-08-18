//! `GET /api/プロジェクト情報`: 開いているプロジェクトのルートパスと名前を返す。

use axum::{Json, extract::State};

use crate::{project_info_contract::プロジェクト情報応答, server_state::サーバー状態};

pub async fn プロジェクト情報を返す(State(状態): State<サーバー状態>) -> Json<プロジェクト情報応答> {
    Json(状態.プロジェクト情報().clone())
}
