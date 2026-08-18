//! 要求のURIから、経路のパーセント符号を解いた新しいURIを組み立てる工程
//! （呼び出し連鎖の中の独立した工程。参照: CLAUDE.md「切り出しの根拠義務」第5項。
//! 入力は元のURI、出力は符号を解いたURIまたは拒否応答）。
//!
//! ブラウザの`fetch`は非ASCIIの経路を必ずパーセント符号化して送るが、axumの経路合わせ
//! （matchit）は届いた生の経路文字列に対して行われ、符号化を解かない。この関数の呼び出し元
//! （`normalized_app/mod.rs`の`経路正規化アプリ`）は経路合わせより先に要求を受け取り、
//! ここで組み立てた新しいURIへ差し替えてから内側のルーターへ渡す。
//!
//! `normalized_app`の子モジュールとして置き、`pub(super)`で親だけへ見せる。

use axum::{
    http::{StatusCode, Uri, uri::PathAndQuery},
    response::{IntoResponse, Response},
};

use crate::failure_response::失敗応答を組み立てる;

use super::percent_decode::経路の符号を解く;

/// 応答は`Response`本体を直接返すと`Result`全体が大きくなる（clippyの
/// `result_large_err`が検出する）ため、拒否応答だけ`Box`で包む。
pub(super) fn 符号を解いた経路を組み立てる(元の経路: &Uri) -> Result<Uri, Box<Response>> {
    let Some(経路と問い合わせ) = 元の経路.path_and_query() else {
        return Ok(元の経路.clone());
    };
    let 解いた経路 = 経路の符号を解く(経路と問い合わせ.path()).map_err(|拒否理由| Box::new(拒否理由.into_response()))?;
    let 新しい経路と問い合わせ文字列 = match 経路と問い合わせ.query() {
        Some(問い合わせ) => format!("{解いた経路}?{問い合わせ}"),
        None => 解いた経路,
    };
    let 新しいパスと問い合わせ: PathAndQuery = 新しい経路と問い合わせ文字列
        .parse()
        .map_err(|_| 経路再構成エラー応答を組み立てる(&新しい経路と問い合わせ文字列))?;

    let mut 部品 = 元の経路.clone().into_parts();
    部品.path_and_query = Some(新しいパスと問い合わせ);
    Uri::from_parts(部品).map_err(|_| 経路再構成エラー応答を組み立てる(&新しい経路と問い合わせ文字列))
}

fn 経路再構成エラー応答を組み立てる(経路と問い合わせ文字列: &str) -> Box<Response> {
    Box::new(失敗応答を組み立てる(
        StatusCode::BAD_REQUEST,
        "経路再構成エラー",
        format!("符号を解いた経路「{経路と問い合わせ文字列}」を要求の経路として組み立て直せない"),
    ))
}
