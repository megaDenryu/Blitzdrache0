//! `PUT /api/建物/{建物定義ID}/格子`: 受け取った格子を解いて建物外形カタログを組み直し、
//! 通ったときだけ正本を書き換える。落ちたときは正本も現在のカタログも動かさない。
//!
//! 経路の識別子と本文が名乗る識別子の一致を課すのは、置き場のディレクトリ名とファイルの名乗りが
//! 食い違った保存物を作らないためである。食い違いは起動時の台帳の読みで初めて露見する。

use axum::{body::Bytes, extract::Path, extract::State, http::StatusCode, response::IntoResponse, response::Response};

use crate::{
    building_grid_store::建物の格子の保存エラー, failure_response::失敗応答を組み立てる, resource::建物の格子, server_state::サーバー状態
};

pub async fn 建物の格子を保存する(
    State(状態): State<サーバー状態>, Path(識別子の綴り): Path<String>, 本文: Bytes
) -> Response {
    match tokio::task::spawn_blocking(move || 検査して保存する(&状態, &識別子の綴り, &本文)).await {
        Ok(Ok(())) => StatusCode::NO_CONTENT.into_response(),
        Ok(Err(応答)) => *応答,
        Err(誤り) => 失敗応答を組み立てる(StatusCode::INTERNAL_SERVER_ERROR, "内部エラー", 誤り.to_string()),
    }
}

/// 応答は`Response`本体を直接返すと`Result`全体が大きくなるため、拒否応答だけ`Box`で包む
/// (`source_asset_export_post`と同じ規律)。
fn 検査して保存する(状態: &サーバー状態, 識別子の綴り: &str, 本文: &[u8]) -> Result<(), Box<Response>> {
    let 格子: 建物の格子 = serde_json::from_slice(本文).map_err(拒否応答へ写す)?;
    if 格子.建物定義ID.綴り() != 識別子の綴り {
        return Err(拒否応答へ写す(
            建物の格子の保存エラー::経路と本文の識別子が食い違う {
                経路の識別子: 識別子の綴り.to_string(),
                本文の識別子: 格子.建物定義ID.綴り().to_string(),
            },
        ));
    }
    let mut 保存係 = 状態.建物の格子の保存係を借りる()?;
    保存係.検査して保存する(&格子).map_err(拒否応答へ写す)
}

fn 拒否応答へ写す(誤り: impl Into<建物の格子の保存エラー>) -> Box<Response> {
    Box::new(誤り.into().into_response())
}
