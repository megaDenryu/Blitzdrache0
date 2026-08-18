//! `GET /api/大域世界/高さ格子`: マザーハイトマップの生バイト列(f32・行優先)を
//! application/octet-streamで返す。未保存なら204番(本文なし)を返す。

use axum::{
    extract::State,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};

use crate::{server_state::サーバー状態, storage::プロジェクト保管庫};

pub async fn 大域世界高さ格子を返す(State(状態): State<サーバー状態>) -> Response {
    match 状態.保管庫().大域世界の高さ格子を読む() {
        Ok(Some(バイト列)) => ([(header::CONTENT_TYPE, "application/octet-stream")], バイト列).into_response(),
        Ok(None) => StatusCode::NO_CONTENT.into_response(),
        Err(エラー) => エラー.into_response(),
    }
}
