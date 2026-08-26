//! `PUT /api/楽曲/{楽曲ID}`: 受け取ったJSONを検証して保存する。
//!
//! 経路の楽曲IDと本文が名乗る楽曲IDの一致を課すのは、置き場のファイル名と正本の中の名乗りが食い違う
//! 保存物を作らないためである。食い違いは以後の読みが拒むため、書ける形にしてはならない
//! (参照: `_doc/設計/楽曲エディター.md`「判断6」)。
//!
//! `Bytes`で受けて手動でJSON解釈するのは、axumの`Json`抽出子が返す拒否応答の形を`{種別, 説明}`へ揃えるためである。

use axum::{body::Bytes, extract::Path, extract::State, http::StatusCode, response::IntoResponse, response::Response};

use crate::{
    failure_response::失敗応答を組み立てる, resource::楽曲, server_state::サーバー状態, storage::プロジェクト保管庫
};

pub async fn 楽曲を保存する(State(状態): State<サーバー状態>, Path(名乗りの綴り): Path<String>, 本文: Bytes) -> Response {
    match 検証して保存する(&状態, &名乗りの綴り, &本文) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(拒否応答) => *拒否応答,
    }
}

/// 拒否応答だけを`Box`で包むのは、`Response`本体を直接返すと`Result`全体が大きくなるためである
/// (`building_grid_put`と同じ規律)。
fn 検証して保存する(状態: &サーバー状態, 名乗りの綴り: &str, 本文: &[u8]) -> Result<(), Box<Response>> {
    let データ: 楽曲 = serde_json::from_slice(本文).map_err(|誤り| {
        Box::new(失敗応答を組み立てる(
            StatusCode::BAD_REQUEST,
            "JSON解析エラー",
            誤り.to_string(),
        ))
    })?;
    if データ.名乗り.綴り() != 名乗りの綴り {
        return Err(Box::new(失敗応答を組み立てる(
            StatusCode::UNPROCESSABLE_ENTITY,
            "識別子不一致エラー",
            format!("経路が指す楽曲ID{名乗りの綴り}と、本文が名乗る{}が食い違う", データ.名乗り),
        )));
    }
    状態
        .保管庫()
        .楽曲を検証して保存する(データ)
        .map_err(|誤り| Box::new(誤り.into_response()))
}
