//! チャンク目録ソースの読込。チャンク世界がどの座標にどのアセットを置くかを宣言したテキストを
//! 検証済みの項目一覧へ変換する。実行時の版付きチャンク目録はこの一覧から作る。
//! 参照: `_doc/設計/チャンクストリーミング.md`「チャンク目録」

mod entry;
mod version;

use std::path::Path;

use blitz_engine::チャンク一辺;
pub use entry::チャンク目録ソース項目;

use crate::error::アセットコンパイルエラー;

/// 先頭行に置く形式名。別形式のテキストを座標宣言として読み進めないための識別子である。
pub struct チャンク目録ソース {
    一辺: チャンク一辺,
    項目一覧: Vec<チャンク目録ソース項目>,
}

impl チャンク目録ソース {
    pub fn 一辺(&self) -> チャンク一辺 {
        self.一辺
    }

    pub fn 項目一覧(self) -> Vec<チャンク目録ソース項目> {
        self.項目一覧
    }
}

pub fn チャンク目録ソースを読み込む(パス: &Path) -> Result<チャンク目録ソース, アセットコンパイルエラー> {
    let 内容 = std::fs::read_to_string(パス)
        .map_err(|誤り| アセットコンパイルエラー::ファイル読込失敗(format!("{}: {誤り}", パス.display())))?;
    let mut 行一覧 = 内容.lines().enumerate();
    let (_, 宣言行) = 行一覧
        .next()
        .ok_or_else(|| アセットコンパイルエラー::チャンク目録ソース形式不正("形式宣言の行が無い".to_string()))?;
    let 一辺 = version::最新の一辺へ変換する(宣言行)?;

    let mut 結果 = Vec::new();
    for (添字, 行) in 行一覧 {
        if 行.trim().is_empty() {
            continue;
        }
        結果.push(チャンク目録ソース項目::行から解析する(添字 + 1, 行)?);
    }
    Ok(チャンク目録ソース {
        一辺, 項目一覧: 結果
    })
}
