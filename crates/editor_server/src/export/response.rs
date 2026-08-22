//! `書き出しエラー`のIntoResponse実装。参照: `failure_response`。

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::error::書き出しエラー;
use crate::failure_response::失敗応答を組み立てる;

impl IntoResponse for 書き出しエラー {
    fn into_response(self) -> Response {
        let (状態, 種別) = 状態と種別を決める(&self);
        失敗応答を組み立てる(状態, 種別, self.to_string())
    }
}

fn 状態と種別を決める(エラー: &書き出しエラー) -> (StatusCode, &'static str) {
    match エラー {
        書き出しエラー::世界名が不正(_) => (StatusCode::BAD_REQUEST, "世界名エラー"),
        書き出しエラー::大域世界が未保存 => (StatusCode::UNPROCESSABLE_ENTITY, "前提条件エラー"),
        書き出しエラー::区画割りの導出に失敗(_) => (StatusCode::UNPROCESSABLE_ENTITY, "構造検証エラー"),
        書き出しエラー::高さ格子処理に失敗(_) => (StatusCode::UNPROCESSABLE_ENTITY, "構造検証エラー"),
        書き出しエラー::マザーからの切り出しに失敗(_) => (StatusCode::UNPROCESSABLE_ENTITY, "構造検証エラー"),
        書き出しエラー::地表材質の重み格子処理に失敗(_) => (StatusCode::UNPROCESSABLE_ENTITY, "構造検証エラー"),
        書き出しエラー::数値変換に失敗(_) => (StatusCode::UNPROCESSABLE_ENTITY, "構造検証エラー"),
        書き出しエラー::建物が所有チャンクの外にある { .. } => (StatusCode::UNPROCESSABLE_ENTITY, "構造検証エラー"),
        書き出しエラー::建物定義が存在しない { .. } => (StatusCode::UNPROCESSABLE_ENTITY, "構造検証エラー"),
        書き出しエラー::散布の個体数が上限を越えている { .. } => (StatusCode::UNPROCESSABLE_ENTITY, "構造検証エラー"),
        書き出しエラー::建物の格子が台帳に無い { .. } => (StatusCode::UNPROCESSABLE_ENTITY, "前提条件エラー"),
        書き出しエラー::建物の格子を読めない(_) => (StatusCode::INTERNAL_SERVER_ERROR, "読み込みエラー"),
        書き出しエラー::アセットIDが不正(_) => (StatusCode::INTERNAL_SERVER_ERROR, "内部エラー"),
        書き出しエラー::読み込みに失敗(_) => (StatusCode::INTERNAL_SERVER_ERROR, "読み込みエラー"),
        書き出しエラー::書き込みに失敗(_) => (StatusCode::INTERNAL_SERVER_ERROR, "保存エラー"),
        書き出しエラー::JSON組み立てに失敗(_) => (StatusCode::INTERNAL_SERVER_ERROR, "内部エラー"),
    }
}
