//! `PUT /api/チャンク/{x}/{z}/高さ格子`: そのチャンクの高さ編集の生バイト列を保存する。
//! 寸法はクエリでなく、保存済みの大域世界の区画割りが導く`チャンク格子の頂点数`と
//! 突き合わせて検証する(`world_layout_lookup`)。

use axum::{
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::world_layout_lookup::保存済み区画割りを読む;
use crate::{
    resource::{チャンクの高さ編集, チャンク座標},
    server_state::サーバー状態,
    storage::プロジェクト保管庫,
    storage::保存要求エラー,
};

pub async fn チャンク高さ格子を保存する(
    State(状態): State<サーバー状態>, Path((x, z)): Path<(i32, i32)>, 本文: Bytes
) -> Response {
    match 検証して保存する(&状態, チャンク座標::生成する(x, z), 本文.to_vec()) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(応答) => *応答,
    }
}

/// 応答は`Response`本体を直接返すと`Result`全体が大きくなる(clippyの
/// `result_large_err`が検出する)ため、拒否応答だけ`Box`で包む。
fn 検証して保存する(状態: &サーバー状態, 座標: チャンク座標, バイト列: Vec<u8>) -> Result<(), Box<Response>> {
    let 区画割り = 保存済み区画割りを読む(状態)?;
    let 頂点数 = 区画割り
        .チャンク格子の頂点数()
        .map_err(|エラー| Box::new(保存要求エラー::検証に失敗(エラー).into_response()))?;
    let 格子 = チャンクの高さ編集::生成する(頂点数, バイト列).map_err(|エラー| Box::new(保存要求エラー::検証に失敗(エラー).into_response()))?;
    状態
        .保管庫()
        .チャンクの高さ格子を検証して保存する(座標, 格子)
        .map_err(|エラー| Box::new(エラー.into_response()))
}
