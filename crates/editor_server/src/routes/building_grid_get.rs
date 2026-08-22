//! `GET /api/建物/{建物定義ID}/格子`: 保存済みの建物の格子を返す。未保存なら200番でJSONの`null`を返す。

use axum::{Json, extract::Path, extract::State, response::IntoResponse, response::Response};

use crate::{failure_response::失敗応答を組み立てる, server_state::サーバー状態};

pub async fn 建物の格子を返す(State(状態): State<サーバー状態>, Path(識別子の綴り): Path<String>) -> Response {
    let 識別子 = match crate::resource::建物定義ID::生成する(識別子の綴り) {
        Ok(識別子) => 識別子,
        Err(誤り) => {
            return 失敗応答を組み立てる(axum::http::StatusCode::BAD_REQUEST, "識別子エラー", 誤り.to_string());
        }
    };
    let 保存係 = match 状態.建物の格子の保存係を借りる() {
        Ok(保存係) => 保存係,
        Err(応答) => return *応答,
    };
    match 保存係.格子を読む(&識別子) {
        Ok(格子) => Json(格子).into_response(),
        Err(誤り) => 誤り.into_response(),
    }
}
