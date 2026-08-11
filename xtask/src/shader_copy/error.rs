//! シェーダーの一時コピーが返す型付きエラー。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//!
//! xtaskは外部のクレートへ依存しないため、thiserrorを使わず手書きのenumと`Display`で書く。
//! 様式は`acceptance/error.rs`に倣う。
//!
//! 3つの枝はどれもファイルの操作の失敗であるが、直す側が見る場所が違う。コピー先を作れないのは書き込み先の権限、
//! 元のディレクトリを読めないのは実行した作業ディレクトリ、1枚を写せないのはそのファイル自身の状態である。

use std::path::PathBuf;

#[derive(Debug)]
pub enum シェーダーの一時コピーの破れ {
    コピー先を作れなかった {
        コピー先: PathBuf,
        誤り: std::io::Error,
    },
    元のディレクトリを読めなかった {
        元のディレクトリ: PathBuf,
        誤り: std::io::Error,
    },
    シェーダーの1枚を写せなかった {
        元パス: PathBuf,
        誤り: std::io::Error,
    },
}

impl std::error::Error for シェーダーの一時コピーの破れ {}

impl std::fmt::Display for シェーダーの一時コピーの破れ {
    fn fmt(&self, 書き手: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::コピー先を作れなかった { コピー先, 誤り } => {
                write!(書き手, "シェーダーのコピー先({})を作れなかった: {誤り}", コピー先.display())
            }
            Self::元のディレクトリを読めなかった {
                元のディレクトリ, 誤り
            } => {
                write!(書き手, "{}の読み取りに失敗した: {誤り}", 元のディレクトリ.display())
            }
            Self::シェーダーの1枚を写せなかった { 元パス, 誤り } => {
                write!(書き手, "{}のコピーに失敗した: {誤り}", 元パス.display())
            }
        }
    }
}
