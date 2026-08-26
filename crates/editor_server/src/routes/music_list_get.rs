//! `GET /api/楽曲一覧`: 保存済みの楽曲の名乗りを昇順で返す。1件も無ければ空の並びを返す。

use axum::{Json, extract::State, response::IntoResponse, response::Response};

use crate::{server_state::サーバー状態, storage::プロジェクト保管庫};

pub async fn 楽曲一覧を返す(State(状態): State<サーバー状態>) -> Response {
    match 状態.保管庫().楽曲の一覧を読む() {
        Ok(名乗り一覧) => Json(名乗り一覧).into_response(),
        Err(誤り) => 誤り.into_response(),
    }
}
