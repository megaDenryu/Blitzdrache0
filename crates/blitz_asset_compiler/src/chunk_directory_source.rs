//! チャンク目録ソースの読込。チャンク世界がどの座標にどのアセットを置くかを宣言したテキストを
//! 検証済みの項目一覧へ変換する。実行時の版付きチャンク目録はこの一覧から作る。
//! 参照: `_doc/設計/チャンクストリーミング.md`「チャンク目録」

mod entry;

use std::path::Path;

pub use entry::チャンク目録ソース項目;

use crate::error::アセットコンパイルエラー;

/// 先頭行に置く形式名。別形式のテキストを座標宣言として読み進めないための識別子である。
const 形式名: &str = "blitz_chunk_directory";
const 対応版: &str = "1";

pub fn チャンク目録ソースを読み込む(
    パス: &Path,
) -> Result<Vec<チャンク目録ソース項目>, アセットコンパイルエラー> {
    let 内容 = std::fs::read_to_string(パス)
        .map_err(|誤り| アセットコンパイルエラー::ファイル読込失敗(format!("{}: {誤り}", パス.display())))?;
    let mut 行一覧 = 内容.lines().enumerate();
    let (_, 宣言行) = 行一覧.next().ok_or_else(|| 形式不正("形式宣言の行が無い"))?;
    形式宣言を検査する(宣言行)?;

    let mut 結果 = Vec::new();
    for (添字, 行) in 行一覧 {
        if 行.trim().is_empty() {
            continue;
        }
        結果.push(チャンク目録ソース項目::行から解析する(添字 + 1, 行)?);
    }
    Ok(結果)
}

/// 先頭行は形式名と版の2欄であり、未知の版は無言で最新として読まずに拒否する。
fn 形式宣言を検査する(宣言行: &str) -> Result<(), アセットコンパイルエラー> {
    let 欄一覧: Vec<&str> = 宣言行.split_whitespace().collect();
    let [名前, 版] = 欄一覧.as_slice() else {
        return Err(形式不正(宣言行));
    };
    if *名前 != 形式名 {
        return Err(形式不正(宣言行));
    }
    if *版 != 対応版 {
        return Err(アセットコンパイルエラー::チャンク目録ソース未対応版(
            (*版).to_string(),
        ));
    }
    Ok(())
}

fn 形式不正(宣言行: &str) -> アセットコンパイルエラー {
    アセットコンパイルエラー::チャンク目録ソース形式不正(宣言行.to_string())
}
