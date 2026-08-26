//! `読み込みエラー`・`保存要求エラー`のIntoResponse実装。参照: `failure_response`。

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::{保存要求エラー, 読み込みエラー};
use crate::failure_response::失敗応答を組み立てる;

impl IntoResponse for 読み込みエラー {
    fn into_response(self) -> Response {
        let 種別 = match &self {
            読み込みエラー::ファイル読み込みに失敗(_) => "読み込みエラー",
            読み込みエラー::Json解釈に失敗(_) => "JSON解析エラー",
            読み込みエラー::旧版移行に失敗(_) => "旧版移行エラー",
            読み込みエラー::楽曲の版を移行できない(_) => "楽曲の版エラー",
            読み込みエラー::ファイル名を名乗りとして読めない { .. } => "置き場の名前エラー",
            読み込みエラー::名乗りとファイル名が食い違う { .. } => "名乗り不一致エラー",
        };
        失敗応答を組み立てる(StatusCode::INTERNAL_SERVER_ERROR, 種別, self.to_string())
    }
}

impl IntoResponse for 保存要求エラー {
    fn into_response(self) -> Response {
        let (状態, 種別) = 状態と種別を決める(&self);
        失敗応答を組み立てる(状態, 種別, self.to_string())
    }
}

fn 状態と種別を決める(エラー: &保存要求エラー) -> (StatusCode, &'static str) {
    match エラー {
        保存要求エラー::既存正本を読めない(_) => (StatusCode::CONFLICT, "既存正本エラー"),
        保存要求エラー::検証に失敗(_) => (StatusCode::UNPROCESSABLE_ENTITY, "構造検証エラー"),
        保存要求エラー::Json処理に失敗(_) => (StatusCode::BAD_REQUEST, "JSON解析エラー"),
        保存要求エラー::書き込みに失敗(_) => (StatusCode::INTERNAL_SERVER_ERROR, "保存エラー"),
    }
}
