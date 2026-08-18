//! `PUT /api/チャンク/{x}/{z}/構造`: 受け取ったJSONを検証して保存する。

use axum::{body::Bytes, extract::Path, extract::State, http::StatusCode, response::IntoResponse, response::Response};

use crate::{
    resource::{チャンク座標, チャンク構造},
    server_state::サーバー状態,
    storage::プロジェクト保管庫,
    storage::保存要求エラー,
};

pub async fn チャンク構造を保存する(State(状態): State<サーバー状態>, Path((x, z)): Path<(i32, i32)>, 本文: Bytes) -> Response {
    match データを検証して保存する(&状態, チャンク座標::生成する(x, z), &本文) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(エラー) => エラー.into_response(),
    }
}

fn データを検証して保存する(状態: &サーバー状態, 座標: チャンク座標, 本文: &[u8]) -> Result<(), 保存要求エラー> {
    let データ: チャンク構造 = serde_json::from_slice(本文)?;
    状態.保管庫().チャンクの構造を検証して保存する(座標, データ)
}
