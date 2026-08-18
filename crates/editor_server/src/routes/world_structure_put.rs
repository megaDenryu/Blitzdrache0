//! `PUT /api/大域世界/構造`: 受け取ったJSONを検証して保存する。
//! `Bytes`で受けて手動でJSON解釈するのは、axumの`Json`抽出子が返す拒否応答の形を
//! `{種別, 説明}`へ揃えるため(手本: GameScriptingTheoryの`routes/spec_put.rs`)。

use axum::{body::Bytes, extract::State, http::StatusCode, response::IntoResponse, response::Response};

use crate::{resource::大域世界構造, server_state::サーバー状態, storage::プロジェクト保管庫, storage::保存要求エラー};

pub async fn 大域世界構造を保存する(State(状態): State<サーバー状態>, 本文: Bytes) -> Response {
    match データを検証して保存する(&状態, &本文) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(エラー) => エラー.into_response(),
    }
}

fn データを検証して保存する(状態: &サーバー状態, 本文: &[u8]) -> Result<(), 保存要求エラー> {
    let データ: 大域世界構造 = serde_json::from_slice(本文)?;
    状態.保管庫().大域世界の構造を検証して保存する(データ)
}
