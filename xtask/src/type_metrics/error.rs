//! 型の計測そのものを実行できなかったことを表す型付きエラー。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//!
//! 計測の結果(型ごとのフィールド数と分散と件数)とは別物である。こちらは計測に到達できなかった側であり、
//! 閾値の判定を持たないこの道具では、走査の材料を読めなかったことだけが破れになる。
//!
//! xtaskは外部のクレートへ依存しないため、thiserrorを使わず手書きのenumと`Display`で書く。様式は`acceptance/error.rs`に倣う。

use std::path::PathBuf;

#[derive(Debug)]
pub enum 型計測の破れ {
    計測対象のファイルを読めなかった { パス: PathBuf, 誤り: std::io::Error },
}

impl std::error::Error for 型計測の破れ {}

impl std::fmt::Display for 型計測の破れ {
    fn fmt(&self, 書き手: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::計測対象のファイルを読めなかった { パス, 誤り } => {
                write!(書き手, "{}の読み取りに失敗した: {誤り}", パス.display())
            }
        }
    }
}
