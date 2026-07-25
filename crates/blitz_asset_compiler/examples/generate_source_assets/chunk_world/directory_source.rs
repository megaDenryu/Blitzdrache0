//! チャンク目録ソースのテキストを組み立てる。`blitz_asset_compiler::チャンク目録ソースを読み込む`が読む側であり、
//! 先頭行の形式宣言と1行4欄(x、z、アセットID、ソース相対パス)の並びをそちらの解析と一致させている。

use blitz_engine::チャンク座標;

use super::{名前を作る, 文書ファイル名を作る};

const 形式宣言: &str = "blitz_chunk_directory 1";

pub(super) fn 目録ソースを作る(座標一覧: &[チャンク座標]) -> String {
    let mut 本文 = String::from(形式宣言);
    本文.push('\n');
    for 座標 in 座標一覧 {
        本文.push_str(&format!(
            "{} {} {} {}\n",
            座標.x(),
            座標.z(),
            名前を作る(*座標),
            文書ファイル名を作る(*座標)
        ));
    }
    本文
}
