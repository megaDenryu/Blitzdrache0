//! `POST /api/書き出し/ソースアセット`: 保管庫が持つ大域世界とチャンクを、blitz_asset_compilerの
//! ソースアセット形式(高さ場+チャンク目録)で`assets/<世界名>/`へ書き出す。
//! ボディ`{ 出力先の世界名: string }`は省略可(既定"editor_world")。
//! 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断5」

use axum::{
    Json,
    body::Bytes,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use blitz_asset_compiler::実行時アセットのコンパイルエラー;
use serde::{Deserialize, Serialize};

use crate::{
    export::{ソースアセット書き出しコマンド, 世界ソース出力先, 出力世界名},
    failure_response::失敗応答を組み立てる,
    server_state::サーバー状態,
};

#[derive(Deserialize, Default)]
struct 要求本体 {
    #[serde(default)]
    出力先の世界名: Option<String>,
}

#[derive(Serialize)]
struct 応答本体 {
    書いたファイル数: usize,
    出力先: String,
}

pub async fn ソースアセットを書き出す(State(状態): State<サーバー状態>, 本文: Bytes) -> Response {
    match tokio::task::spawn_blocking(move || ソースアセットの書き出しとベイクを実行する(&状態, &本文)).await {
        Ok(Ok(応答)) => Json(応答).into_response(),
        Ok(Err(応答)) => *応答,
        Err(誤り) => 失敗応答を組み立てる(StatusCode::INTERNAL_SERVER_ERROR, "内部エラー", 誤り.to_string()),
    }
}

/// 応答は`Response`本体を直接返すと`Result`全体が大きくなる(clippyの`result_large_err`が検出する)ため、拒否応答だけ`Box`で包む。
fn ソースアセットの書き出しとベイクを実行する(
    状態: &サーバー状態, 本文: &[u8]
) -> Result<応答本体, Box<Response>> {
    let _排他権 = 状態.ソースアセット書き出しの排他権().lock().map_err(|誤り| {
        Box::new(失敗応答を組み立てる(
            StatusCode::INTERNAL_SERVER_ERROR,
            "内部エラー",
            誤り.to_string(),
        ))
    })?;
    let 要求 = 要求本体を解く(本文)?;
    let 世界名 = 出力世界名::生成する(要求.出力先の世界名).map_err(|エラー| Box::new(エラー.into_response()))?;
    if !世界名.エディターの世界か() {
        return Err(Box::new(失敗応答を組み立てる(
            StatusCode::BAD_REQUEST,
            "世界名エラー",
            "同期ベイクの対象はeditor_worldだけである".to_string(),
        )));
    }
    let 出力先 = 世界ソース出力先::生成する(状態.リポジトリルート(), &世界名);
    // カタログと格子を同じ錠の中で写し取るのは、書き出しの最中に保存が走っても、この1回が同じ世代のカタログと格子だけを見るようにするためである。
    let (カタログ, 格子の台帳) = {
        let 保存係 = 状態.建物の格子の保存係を借りる()?;
        (保存係.現在のカタログ().clone(), 保存係.格子の台帳().clone())
    };
    let コマンド = ソースアセット書き出しコマンド::生成する(状態.保管庫().clone(), 出力先, カタログ, 格子の台帳);
    let 結果 = コマンド.実行する().map_err(|エラー| Box::new(エラー.into_response()))?;
    状態.エディターの世界をベイクする().map_err(ベイク失敗の応答)?;
    Ok(応答本体 {
        書いたファイル数: 結果.書いたファイル数(),
        出力先: 結果.出力先ディレクトリ().to_string(),
    })
}

fn ベイク失敗の応答(誤り: 実行時アセットのコンパイルエラー) -> Box<Response> {
    let 状態 = match 誤り {
        実行時アセットのコンパイルエラー::ソース不正(_) => StatusCode::UNPROCESSABLE_ENTITY,
        実行時アセットのコンパイルエラー::入出力(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
    Box::new(失敗応答を組み立てる(
        状態,
        "ベイクエラー",
        format!("ソースアセットは書き出したが、ベイクに失敗した: {誤り}"),
    ))
}

fn 要求本体を解く(本文: &[u8]) -> Result<要求本体, Box<Response>> {
    if 本文.is_empty() {
        return Ok(要求本体::default());
    }
    serde_json::from_slice(本文).map_err(|誤り| {
        Box::new(失敗応答を組み立てる(
            StatusCode::BAD_REQUEST,
            "JSON解析エラー",
            誤り.to_string(),
        ))
    })
}
