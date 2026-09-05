//! ビルドの中間データの掃除が返す型付きエラー。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//! 様式は`file_scan/error.rs`に倣い、外部クレートへ依存せず手書きの列挙と`Display`で書く。

use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum 掃除の破れ {
    ディレクトリを読めなかった { ディレクトリ: PathBuf, 誤り: std::io::Error },
    ディレクトリを消せなかった { ディレクトリ: PathBuf, 誤り: std::io::Error },
}

impl std::error::Error for 掃除の破れ {}

impl std::fmt::Display for 掃除の破れ {
    fn fmt(&self, 書き手: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ディレクトリを読めなかった { ディレクトリ, 誤り } => {
                write!(書き手, "{}の読み取りに失敗した: {誤り}", ディレクトリ.display())
            }
            Self::ディレクトリを消せなかった { ディレクトリ, 誤り } => {
                write!(書き手, "{}の削除に失敗した: {誤り}", ディレクトリ.display())
            }
        }
    }
}
