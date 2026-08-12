//! 検査対象ファイルの走査が返す型付きエラー。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//!
//! xtaskは外部のクレートへ依存しないため、thiserrorを使わず手書きのenumと`Display`で書く。
//! 様式は`acceptance/error.rs`に倣う。
//!
//! 呼び出し側は自分の誤りの型がこの破れを枝で内包し、`?`で受ける(規約検査の破れ・検収エラー)。
//! 生の文へ戻る変換をこの型が持たないのは、この走査を呼ぶ入口がすべて型付きエラーへ移り終えたためである。

use std::path::PathBuf;

#[derive(Debug)]
pub enum ファイル走査の破れ {
    ディレクトリを読めなかった { ディレクトリ: PathBuf, 誤り: std::io::Error },
}

impl std::error::Error for ファイル走査の破れ {}

impl std::fmt::Display for ファイル走査の破れ {
    fn fmt(&self, 書き手: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ディレクトリを読めなかった { ディレクトリ, 誤り } => {
                write!(書き手, "{}の読み取りに失敗した: {誤り}", ディレクトリ.display())
            }
        }
    }
}
