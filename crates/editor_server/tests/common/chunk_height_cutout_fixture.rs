//! `chunk_heightmap_mother_cutout*`試験が共有する、大域の用意と高さ格子の取り出し。
//! 小さな区画割り(2チャンク四方・解像度2)を前提に、マザーの格子点の値を綴りひとつで言い表す。
#![allow(clippy::unwrap_used)]
#![allow(dead_code)]

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use super::一時プロジェクト;

/// 小さな区画割りでのマザーハイトマップの1辺の頂点数。
pub const マザー一辺頂点数: usize = 5;
/// 小さな区画割りでのチャンク単独の編集格子の1辺の頂点数。
pub const チャンク一辺頂点数: usize = 3;

pub async fn 高さ格子を取得する(一時: &一時プロジェクト, x: i32, z: i32) -> (StatusCode, Vec<u8>) {
    let 応答 = super::ルーターを作る(一時)
        .oneshot(Request::get(format!("/api/チャンク/{x}/{z}/高さ格子")).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let 状態 = 応答.status();
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    (状態, 本体.to_vec())
}

pub fn 高さ一覧へ解く(バイト列: &[u8]) -> Vec<f32> {
    バイト列
        .chunks_exact(4)
        .map(|区切り| f32::from_le_bytes(<[u8; 4]>::try_from(区切り).unwrap()))
        .collect()
}

/// `マザーを一意な値で保存する`が埋める`高さ(x, z) = z*一辺頂点数 + x`の値。
pub fn マザーの高さ(大域x: usize, 大域z: usize) -> f32 {
    f32::from(u16::try_from(大域z * マザー一辺頂点数 + 大域x).unwrap())
}

pub fn 大域を一意な値で用意する(一時: &一時プロジェクト) {
    let 保管庫 = editor_server::ファイル保管庫::生成する(&一時.プロジェクトルート());
    let 区画割り = super::小さな区画割り();
    super::大域世界を保存する(&保管庫, 区画割り);
    super::マザーを一意な値で保存する(&保管庫, 区画割り);
}
