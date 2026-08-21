//! `PUT /api/チャンク/{x}/{z}/構造`: 受け取ったJSONを検証して保存する。

use axum::{body::Bytes, extract::Path, extract::State, http::StatusCode, response::IntoResponse, response::Response};

use crate::{
    resource::{チャンク座標, チャンク構造, 資源検証エラー},
    server_state::サーバー状態,
    storage::プロジェクト保管庫,
    storage::保存要求エラー,
};

pub async fn チャンク構造を保存する(State(状態): State<サーバー状態>, Path((x, z)): Path<(i32, i32)>, 本文: Bytes) -> Response {
    match データを検証して保存する(&状態, チャンク座標::生成する(x, z), &本文) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(エラー) => エラー.into_response(),
    }
}

fn データを検証して保存する(状態: &サーバー状態, 座標: チャンク座標, 本文: &[u8]) -> Result<(), 保存要求エラー> {
    let データ: チャンク構造 = serde_json::from_slice(本文)?;
    for 建物 in &データ.建物一覧 {
        if !状態.建物外形カタログ().建物定義を含む(&建物.建物定義ID) {
            return Err(資源検証エラー::参照先が存在しない {
                フィールド名: "建物の配置.建物定義ID",
                値: 建物.建物定義ID.clone(),
            }
            .into());
        }
    }
    状態.保管庫().チャンクの構造を検証して保存する(座標, データ)
}
