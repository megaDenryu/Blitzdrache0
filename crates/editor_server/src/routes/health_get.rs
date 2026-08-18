//! `GET /api/生存確認`: 編集サーバーが起動していることを示す固定の応答を返す。

use axum::Json;

use crate::health_contract::生存確認応答;

pub async fn 生存確認を返す() -> Json<生存確認応答> {
    Json(生存確認応答::生成する())
}
