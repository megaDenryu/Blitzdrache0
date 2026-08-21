//! チャンク目録ソースのテキストを組み立てる。`blitz_asset_compiler::チャンク目録ソースを読み込む`が読む側であり、
//! 先頭行の形式宣言と1行4欄(x、z、アセット識別子、ソース相対パス)の並びをそちらの解析と一致させている。
//! 板の世界と地形の世界のどちらもこの形式で目録を宣言するため、書き手はこの1箇所に集める。

use blitz_engine::チャンク座標;

pub(crate) struct 目録項目 {
    pub(crate) 座標: チャンク座標,
    pub(crate) アセット識別子: String,
    pub(crate) ソース相対パス: String,
}

pub(crate) fn 目録ソースを作る(一辺メートル: f32, 項目一覧: &[目録項目]) -> String {
    let mut 本文 = format!("blitz_chunk_directory 2 {一辺メートル}\n");
    for 項目 in 項目一覧 {
        本文.push_str(&format!(
            "{} {} {} {}\n",
            項目.座標.x(),
            項目.座標.z(),
            項目.アセット識別子,
            項目.ソース相対パス
        ));
    }
    本文
}
