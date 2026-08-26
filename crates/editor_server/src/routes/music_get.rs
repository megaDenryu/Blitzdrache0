//! `GET /api/楽曲/{楽曲ID}`: 保存済みの楽曲を返す。未保存なら200番でJSONの`null`を返す。

use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse, response::Response};

use crate::{
    failure_response::失敗応答を組み立てる, resource::楽曲ID, server_state::サーバー状態, storage::プロジェクト保管庫
};

pub async fn 楽曲を返す(State(状態): State<サーバー状態>, Path(名乗りの綴り): Path<String>) -> Response {
    let 名乗り = match 楽曲ID::生成する(名乗りの綴り) {
        Ok(名乗り) => 名乗り,
        Err(誤り) => return 失敗応答を組み立てる(StatusCode::BAD_REQUEST, "識別子エラー", 誤り.to_string()),
    };
    match 状態.保管庫().楽曲を読む(&名乗り) {
        Ok(データ) => Json(データ).into_response(),
        Err(誤り) => 誤り.into_response(),
    }
}
