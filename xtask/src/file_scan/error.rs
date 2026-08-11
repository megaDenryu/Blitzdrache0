//! 検査対象ファイルの走査が返す型付きエラー。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//!
//! xtaskは外部のクレートへ依存しないため、thiserrorを使わず手書きのenumと`Display`で書く。
//! 様式は`acceptance/error.rs`に倣う。
//!
//! 生の文へ戻る変換を1つ置くのは、この走査を呼ぶconformと型計測の入口がまだ`Result<_, String>`を返すためである。
//! 規約検査は検収の判定層ではないため、その2つの入口の型付けはこの工程の型付けとは別に行う。

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

/// 生の文へ戻る唯一の境界。`Result<_, String>`を返すconformと型計測の入口が`?`でそのまま受けられるようにする。
impl From<ファイル走査の破れ> for String {
    fn from(破れ: ファイル走査の破れ) -> Self {
        破れ.to_string()
    }
}
