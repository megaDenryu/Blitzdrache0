//! XPBDの並列方式の計測の失敗。検収の器の破れと判定の破れを枝で内包するのは、この入口が起動と読み取りと判定と
//! ファイルの書き出しを一続きに通すためである。人が読む1文への写しもここが持つ(枝が少なく別ファイルにする理由が無い)。

use std::fmt;
use std::path::PathBuf;

use crate::acceptance::{判定の破れ, 検収エラー};

#[derive(Debug)]
pub(super) enum XPBDの並列方式の計測エラー {
    検収の器が破れた(検収エラー),
    判定が破れた(判定の破れ),
    知らない引数を渡された { 語: String },
    引数の次に値が無い { 引数名: &'static str },
    引数の値を読めない { 引数名: &'static str, 語: String },
    数が零である { 引数名: &'static str },
    出力先を作れなかった { 誤り: std::io::Error },
    実行の標準出力を書けなかった { パス: PathBuf, 誤り: std::io::Error },
    計測の結果を書けなかった { パス: PathBuf, 誤り: std::io::Error },
    計測用の構築が失敗した(crate::release_build::計測用の構築の破れ),
}

impl std::error::Error for XPBDの並列方式の計測エラー {}

impl From<検収エラー> for XPBDの並列方式の計測エラー {
    fn from(破れ: 検収エラー) -> Self {
        Self::検収の器が破れた(破れ)
    }
}

impl From<判定の破れ> for XPBDの並列方式の計測エラー {
    fn from(破れ: 判定の破れ) -> Self {
        Self::判定が破れた(破れ)
    }
}

impl fmt::Display for XPBDの並列方式の計測エラー {
    fn fmt(&self, 書き手: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::検収の器が破れた(破れ) => write!(書き手, "{破れ}"),
            Self::判定が破れた(破れ) => write!(書き手, "{破れ}"),
            Self::知らない引数を渡された { 語 } => write!(
                書き手,
                "知らない引数({語})。使えるのは--method・--graph・--iterations・--steps・--points・--compare-stepsである"
            ),
            Self::引数の次に値が無い { 引数名 } => write!(書き手, "{引数名}の次に値が無い"),
            Self::引数の値を読めない { 引数名, 語 } => write!(書き手, "{引数名}の値を読めない({語})"),
            Self::数が零である { 引数名 } => write!(書き手, "{引数名}は1以上である必要がある"),
            Self::出力先を作れなかった { 誤り } => write!(書き手, "出力先を作れなかった: {誤り}"),
            Self::実行の標準出力を書けなかった { パス, 誤り } | Self::計測の結果を書けなかった { パス, 誤り } => {
                write!(書き手, "{}を書けなかった: {誤り}", パス.display())
            }
            Self::計測用の構築が失敗した(破れ) => write!(書き手, "{破れ}"),
        }
    }
}
