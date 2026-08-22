//! `GET /api/建物一覧`: 格子を保存済みの建物を識別子の昇順で返す。1件も無ければ空の並びを返す。

use axum::{Json, extract::State, response::IntoResponse, response::Response};

use crate::server_state::サーバー状態;

pub async fn 建物一覧を返す(State(状態): State<サーバー状態>) -> Response {
    match 状態.建物の格子の保存係を借りる() {
        Ok(保存係) => Json(保存係.一覧を並べる()).into_response(),
        Err(応答) => *応答,
    }
}
