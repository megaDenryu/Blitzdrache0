//! `GET /api/チャンク/{x}/{z}/材質重み`: そのチャンクの地表材質の重みの生バイト列(u8×4層)を
//! application/octet-streamで返す。未保存なら204番(本文なし)を返す。

use axum::{
    extract::{Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};

use crate::{resource::チャンク座標, server_state::サーバー状態, storage::プロジェクト保管庫};

pub async fn チャンク材質重みを返す(State(状態): State<サーバー状態>, Path((x, z)): Path<(i32, i32)>) -> Response {
    match 状態.保管庫().チャンクの材質重みを読む(チャンク座標::生成する(x, z)) {
        Ok(Some(バイト列)) => ([(header::CONTENT_TYPE, "application/octet-stream")], バイト列).into_response(),
        Ok(None) => StatusCode::NO_CONTENT.into_response(),
        Err(エラー) => エラー.into_response(),
    }
}
