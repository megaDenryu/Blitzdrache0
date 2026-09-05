//! スモークのアセットの一時コピーが返す型付きエラー。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//!
//! xtaskは外部のクレートへ依存しないため、thiserrorを使わず手書きのenumと`Display`で書く。
//! 様式は`acceptance/error.rs`に倣う。

use std::path::PathBuf;

#[derive(Debug)]
pub(in crate::smoke) enum スモークのアセットの一時コピーの破れ {
    コピー先を作れなかった {
        コピー先: PathBuf,
        誤り: std::io::Error,
    },
    元のディレクトリを読めなかった {
        元のディレクトリ: PathBuf,
        誤り: std::io::Error,
    },
    アセットの1枚を写せなかった {
        元パス: PathBuf,
        誤り: std::io::Error,
    },
    生成物のソースアセットが揃わなかった,
}

impl std::error::Error for スモークのアセットの一時コピーの破れ {}

impl std::fmt::Display for スモークのアセットの一時コピーの破れ {
    fn fmt(&self, 書き手: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::コピー先を作れなかった { コピー先, 誤り } => {
                write!(書き手, "コピー先ディレクトリ({})の作成に失敗した: {誤り}", コピー先.display())
            }
            Self::元のディレクトリを読めなかった {
                元のディレクトリ, 誤り
            } => {
                write!(書き手, "{}の読み取りに失敗した: {誤り}", 元のディレクトリ.display())
            }
            Self::アセットの1枚を写せなかった { 元パス, 誤り } => {
                write!(書き手, "{}のコピーに失敗した: {誤り}", 元パス.display())
            }
            Self::生成物のソースアセットが揃わなかった => {
                write!(書き手, "写す元の共有バッファを揃えられなかった(理由は直前の行が名指している)")
            }
        }
    }
}
