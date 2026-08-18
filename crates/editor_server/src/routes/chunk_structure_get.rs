//! `GET /api/チャンク/{x}/{z}/構造`: 保存済みのチャンク構造を返す。未保存なら200番でJSONの`null`を返す。

use axum::{Json, extract::Path, extract::State, response::IntoResponse, response::Response};

use crate::{resource::チャンク座標, server_state::サーバー状態, storage::プロジェクト保管庫};

pub async fn チャンク構造を返す(State(状態): State<サーバー状態>, Path((x, z)): Path<(i32, i32)>) -> Response {
    match 状態.保管庫().チャンクの構造を読む(チャンク座標::生成する(x, z)) {
        Ok(データ) => Json(データ).into_response(),
        Err(エラー) => エラー.into_response(),
    }
}
