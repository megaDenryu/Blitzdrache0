//! `GET /api/チャンク/{x}/{z}/高さ格子`: そのチャンクの高さ編集の生バイト列(f32・行優先)を
//! application/octet-streamで返す。まだ保存されていないチャンクへは、大域のマザーハイトマップから
//! 切り出した初期値を返す(導出は`chunk_height_supply`が持つ)。大域も未保存なら204番(本文なし)を返す。

use axum::{
    extract::{Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};

use crate::{resource::チャンク座標, server_state::サーバー状態};

pub async fn チャンク高さ格子を返す(State(状態): State<サーバー状態>, Path((x, z)): Path<(i32, i32)>) -> Response {
    match 状態.チャンクの高さ格子を読み未保存ならマザーから切り出す(チャンク座標::生成する(x, z)) {
        Ok(Some(バイト列)) => ([(header::CONTENT_TYPE, "application/octet-stream")], バイト列).into_response(),
        Ok(None) => StatusCode::NO_CONTENT.into_response(),
        Err(エラー) => エラー.into_response(),
    }
}
