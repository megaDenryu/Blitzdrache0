//! 目視用の絵への変換が返す型付きエラー。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//!
//! xtaskは外部のクレートへ依存しないため、thiserrorを使わず手書きのenumと`Display`で書く。
//! 様式は`acceptance/error.rs`に倣う。
//!
//! 変換ツールの起動を「起こせなかった」と「失敗して終わった」に分けるのは、直す側が見る場所が違うためである。
//! 前者はImageMagickが実行環境に入っておらず、後者は渡した寸法か生バイト列が食い違っている。

use std::path::PathBuf;

#[derive(Debug)]
pub enum 目視用の絵への変換の破れ {
    読み戻し寸法を読めなかった { 寸法のパス: PathBuf, 誤り: std::io::Error },
    変換ツールを起こせなかった { 誤り: std::io::Error },
    変換ツールが失敗して終わった { 終了状態: String },
    絵の絶対パスを取れなかった { 絵のパス: PathBuf, 誤り: std::io::Error },
}

impl std::error::Error for 目視用の絵への変換の破れ {}

impl std::fmt::Display for 目視用の絵への変換の破れ {
    fn fmt(&self, 書き手: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::読み戻し寸法を読めなかった { 寸法のパス, 誤り } => {
                write!(書き手, "読み戻し寸法({})を読めなかった: {誤り}", 寸法のパス.display())
            }
            Self::変換ツールを起こせなかった { 誤り } => {
                write!(書き手, "ImageMagickを起動できなかった: {誤り}")
            }
            Self::変換ツールが失敗して終わった { 終了状態 } => {
                write!(書き手, "ImageMagickが{終了状態}で失敗した")
            }
            Self::絵の絶対パスを取れなかった { 絵のパス, 誤り } => {
                write!(書き手, "{}の絶対パスを取れなかった: {誤り}", 絵のパス.display())
            }
        }
    }
}
