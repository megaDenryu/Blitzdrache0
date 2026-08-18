//! `経路符号を解く際の拒否理由`のIntoResponse実装。外部トレイトの実装を型自身の
//! 操作から分ける（参照: CLAUDE.md「切り出しの根拠義務」第1号）。
//! いずれの拒否理由も、届いた要求そのものが誤った形をしているために起きるため400を返す。

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::failure_response::失敗応答を組み立てる;

use super::percent_decode::経路符号を解く際の拒否理由;

impl IntoResponse for 経路符号を解く際の拒否理由 {
    fn into_response(self) -> Response {
        失敗応答を組み立てる(StatusCode::BAD_REQUEST, "経路符号エラー", self.to_string())
    }
}
