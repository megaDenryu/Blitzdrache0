//! 失敗をJSON応答`{ 種別, 説明 }`へ変換する共通の組み立て。
//! 段1で使うのは経路の符号を解く工程の拒否応答だけだが、段2以降の保管庫実装が
//! 同じ形の応答を返すため、失敗応答の形をここへ1箇所へ集約する。

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
struct 失敗応答本文 {
    種別: String,
    説明: String,
}

pub fn 失敗応答を組み立てる(状態: StatusCode, 種別: &str, 説明: String) -> Response {
    (
        状態,
        Json(失敗応答本文 {
            種別: 種別.to_string(),
            説明,
        }),
    )
        .into_response()
}
