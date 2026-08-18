//! 格子系のPUT経路(大域世界・チャンクの高さ格子、材質重み)が共通で使う、
//! 保存済みの区画割りの取り出し。区画割りが無い間は格子を受け付けない
//! (寸法をクエリでなく構造側の区画割りと突き合わせて検証する、という設計判断の実装)。

use axum::{http::StatusCode, response::IntoResponse, response::Response};

use crate::{
    failure_response::失敗応答を組み立てる, resource::世界の区画割り, server_state::サーバー状態, storage::プロジェクト保管庫
};

/// 応答は`Response`本体を直接返すと`Result`全体が大きくなる(clippyの
/// `result_large_err`が検出する)ため、拒否応答だけ`Box`で包む。
pub(super) fn 保存済み区画割りを読む(状態: &サーバー状態) -> Result<世界の区画割り, Box<Response>> {
    let 構造 = 状態.保管庫().大域世界の構造を読む().map_err(|エラー| Box::new(エラー.into_response()))?;
    構造.map(|構造| 構造.区画割り).ok_or_else(|| Box::new(区画割り未設定応答()))
}

fn 区画割り未設定応答() -> Response {
    失敗応答を組み立てる(
        StatusCode::BAD_REQUEST,
        "前提条件エラー",
        "大域世界の構造(区画割り)がまだ保存されていない。先に大域世界の構造を保存する".to_string(),
    )
}
