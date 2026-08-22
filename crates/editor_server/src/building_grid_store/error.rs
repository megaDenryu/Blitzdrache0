//! 建物の格子の保存と読みで起きる失敗と、その失敗をHTTPの応答へ写す規律。
//!
//! 状態番号を枝ごとに決めるのは、送り手が直せる破れ(格子の宣言・欄の値)と、送り手にはどうにもならない破れ
//! (外部アセットの置き場が無い・ファイルへ書けない)を、受け手が区別できるようにするためである。

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::failure_response::失敗応答を組み立てる;
use crate::resource::資源検証エラー;

#[derive(Debug, thiserror::Error)]
pub enum 建物の格子の保存エラー {
    #[error("要求の本文をJSONとして解釈できない: {0}")]
    Json解釈に失敗(#[from] serde_json::Error),

    #[error("受け取った内容が型契約の不変条件を満たさない: {0}")]
    検証に失敗(#[from] 資源検証エラー),

    #[error("経路が指す建物定義ID{経路の識別子}と、本文が名乗る{本文の識別子}が食い違う")]
    経路と本文の識別子が食い違う {
        経路の識別子: String, 本文の識別子: String
    },

    #[error("建物の格子を型契約からソースの形へ写せない: {0}")]
    ソースの形へ写せない(String),

    #[error("建物の格子が組み立てに耐えない: {0}")]
    格子を解けない(#[from] blitz_asset_compiler::建物の格子のソースエラー),

    #[error("建物外形カタログを組み直せない: {0}")]
    カタログを組み直せない(#[from] blitz_asset_compiler::建物外形カタログエラー),

    #[error("組み直した建物外形カタログを読み戻せない: {0}")]
    カタログを読み戻せない(#[from] crate::resource::建物外形カタログ読み込みエラー),
}

impl IntoResponse for 建物の格子の保存エラー {
    fn into_response(self) -> Response {
        let (状態, 種別) = 状態と種別を決める(&self);
        失敗応答を組み立てる(状態, 種別, self.to_string())
    }
}

fn 状態と種別を決める(エラー: &建物の格子の保存エラー) -> (StatusCode, &'static str) {
    match エラー {
        建物の格子の保存エラー::Json解釈に失敗(_) => (StatusCode::BAD_REQUEST, "JSON解析エラー"),
        建物の格子の保存エラー::検証に失敗(_) => (StatusCode::UNPROCESSABLE_ENTITY, "構造検証エラー"),
        建物の格子の保存エラー::経路と本文の識別子が食い違う { .. } => (StatusCode::BAD_REQUEST, "識別子不一致エラー"),
        建物の格子の保存エラー::ソースの形へ写せない(_) => (StatusCode::UNPROCESSABLE_ENTITY, "構造検証エラー"),
        建物の格子の保存エラー::格子を解けない(原因) => 格子の破れの状態と種別(原因),
        建物の格子の保存エラー::カタログを組み直せない(原因) => カタログの破れの状態と種別(原因),
        建物の格子の保存エラー::カタログを読み戻せない(_) => (StatusCode::INTERNAL_SERVER_ERROR, "カタログ読み込みエラー"),
    }
}

/// ファイルシステムに触れて起きた破れは送り手に直せない。宣言そのものの破れだけを400番台にする。
fn 格子の破れの状態と種別(原因: &blitz_asset_compiler::建物の格子のソースエラー) -> (StatusCode, &'static str) {
    use blitz_asset_compiler::建物の格子のソースエラー as 格子の破れ;
    match 原因 {
        格子の破れ::置き場を読めない { .. }
        | 格子の破れ::ファイルを読めない { .. }
        | 格子の破れ::書き出せない { .. }
        | 格子の破れ::Jsonを組み立てられない { .. } => (StatusCode::INTERNAL_SERVER_ERROR, "格子保存エラー"),
        _ => (StatusCode::UNPROCESSABLE_ENTITY, "格子検証エラー"),
    }
}

/// 外部アセットの置き場が無い環境では、どんな格子を送っても組み立てが通らない。送り手の破れではない。
fn カタログの破れの状態と種別(原因: &blitz_asset_compiler::建物外形カタログエラー) -> (StatusCode, &'static str) {
    match 原因 {
        blitz_asset_compiler::建物外形カタログエラー::外部ソースルート不在(_)
        | blitz_asset_compiler::建物外形カタログエラー::ファイルを書き込めない { .. } => {
            (StatusCode::INTERNAL_SERVER_ERROR, "部品の置き場エラー")
        }
        _ => (StatusCode::UNPROCESSABLE_ENTITY, "カタログ組み立てエラー"),
    }
}
