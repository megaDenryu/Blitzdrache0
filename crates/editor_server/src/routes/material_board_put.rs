//! `PUT /api/マテリアル台帳`: 受け取ったJSONを検証して保存する。
//! `Bytes`で受けて手動でJSON解釈するのは、axumの`Json`抽出子が返す拒否応答の形を
//! `{種別, 説明}`へ揃えるため(手本: `world_structure_put`)。

use axum::{body::Bytes, extract::State, http::StatusCode, response::IntoResponse, response::Response};

use crate::{
    resource::マテリアル台帳, server_state::サーバー状態, storage::プロジェクト保管庫, storage::保存要求エラー
};

pub async fn マテリアル台帳を保存する(State(状態): State<サーバー状態>, 本文: Bytes) -> Response {
    match データを検証して保存する(&状態, &本文) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(エラー) => エラー.into_response(),
    }
}

fn データを検証して保存する(状態: &サーバー状態, 本文: &[u8]) -> Result<(), 保存要求エラー> {
    let データ: マテリアル台帳 = serde_json::from_slice(本文)?;
    状態.保管庫().マテリアル台帳を検証して保存する(データ)
}
