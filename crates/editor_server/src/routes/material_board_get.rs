//! `GET /api/マテリアル台帳`: 保存済みのマテリアル台帳を返す。未保存なら200番でJSONの`null`を返す。

use axum::{Json, extract::State, response::IntoResponse, response::Response};

use crate::{server_state::サーバー状態, storage::プロジェクト保管庫};

pub async fn マテリアル台帳を返す(State(状態): State<サーバー状態>) -> Response {
    match 状態.保管庫().マテリアル台帳を読む() {
        Ok(データ) => Json(データ).into_response(),
        Err(エラー) => エラー.into_response(),
    }
}
