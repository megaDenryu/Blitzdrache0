//! 型の計測そのものを実行できなかったことを表す型付きエラー。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//!
//! 計測の結果(型ごとのフィールド数と分散と件数)とは別物である。こちらは計測に到達できなかった側であり、
//! 走査の材料を読めなかったことだけが破れになる。
//!
//! xtaskは外部のクレートへ依存しないため、thiserrorを使わず手書きのenumと`Display`で書く。様式は`acceptance/error.rs`に倣う。

use std::path::PathBuf;

use crate::file_scan::ファイル走査の破れ;

#[derive(Debug)]
pub enum 型計測の破れ {
    計測対象のファイルを読めなかった { パス: PathBuf, 誤り: std::io::Error },
    計測対象のファイルを走査できなかった(ファイル走査の破れ),
}

impl std::error::Error for 型計測の破れ {}

/// ファイルの走査の破れを型計測の破れへ載せる境界。走査の側は自分が型計測の一部であることを知らない。
impl From<ファイル走査の破れ> for 型計測の破れ {
    fn from(破れ: ファイル走査の破れ) -> Self {
        Self::計測対象のファイルを走査できなかった(破れ)
    }
}

impl std::fmt::Display for 型計測の破れ {
    fn fmt(&self, 書き手: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::計測対象のファイルを読めなかった { パス, 誤り } => {
                write!(書き手, "{}の読み取りに失敗した: {誤り}", パス.display())
            }
            Self::計測対象のファイルを走査できなかった(破れ) => {
                write!(書き手, "計測対象のファイルを走査できなかった: {破れ}")
            }
        }
    }
}
