//! `GET /api/大域世界/構造`: 保存済みの大域世界構造を返す。未保存なら200番でJSONの`null`を返す。

use axum::{Json, extract::State, response::IntoResponse, response::Response};

use crate::{server_state::サーバー状態, storage::プロジェクト保管庫};

pub async fn 大域世界構造を返す(State(状態): State<サーバー状態>) -> Response {
    match 状態.保管庫().大域世界の構造を読む() {
        Ok(データ) => Json(データ).into_response(),
        Err(エラー) => エラー.into_response(),
    }
}
