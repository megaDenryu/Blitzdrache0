//! 散布の検査が使う、書き出しの実行と生成物の読み取り。担当するのは、一時プロジェクトを整えて
//! `POST /api/書き出し/ソースアセット`を1回通すことと、その結果できたチャンクソースと焼いたシーンを読むことである。
#![allow(clippy::unwrap_used)]

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde::Deserialize;
use tower::ServiceExt;

/// 書き出したJSONのうち、検査が読む欄だけを写した形。書き手の型をそのまま使わないのは、
/// 書き手が欄の名前を変えたときに検査が黙って通らないようにするためである。
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct 読み取る散布の群 {
    pub 植生定義ID: String,
    pub 個体一覧: Vec<読み取る散布の個体>,
}

#[derive(Deserialize)]
pub struct 読み取る散布の個体 {
    pub 配置識別子: String,
    pub チャンク原点からの東メートル: f64,
    pub チャンク原点からの南メートル: f64,
}

#[derive(Deserialize)]
pub struct 読み取るチャンクソース {
    pub 形式版: u32,
    pub 散布の群一覧: Vec<読み取る散布の群>,
}

pub async fn 散布を載せて書き出す(識別子: &str) -> super::一時プロジェクト {
    let 一時 = super::一時プロジェクト::生成する(識別子);
    let 保管庫 = editor_server::ファイル保管庫::生成する(&一時.プロジェクトルート());
    let 区画割り = super::建物を据えられる区画割り();
    super::大域世界を保存する(&保管庫, 区画割り);
    super::零のマザーを保存する(&保管庫, 区画割り);
    super::フォックスのソースを配置する(&一時);
    super::地表層のタイルを配置する(&一時);
    super::散布を載せたチャンク構造を保存する(&保管庫, editor_server::チャンク座標::生成する(0, 0));
    let 応答 = super::ルーターを作る(&一時)
        .oneshot(Request::post("/api/書き出し/ソースアセット").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(応答.status(), StatusCode::OK);
    一時
}

pub fn 書き出したチャンクソースを読む(一時: &super::一時プロジェクト) -> 読み取るチャンクソース {
    let パス = 一時.ルート().join("assets/editor_world/editor_chunk_x0_z0.json");
    serde_json::from_slice(&std::fs::read(&パス).unwrap()).unwrap()
}

pub fn 焼いたチャンクのバイト列を読む(一時: &super::一時プロジェクト) -> Vec<u8> {
    std::fs::read(一時.ルート().join("target/editor_world_assets/editor_terrain_x0_z0.blitzasset")).unwrap()
}
