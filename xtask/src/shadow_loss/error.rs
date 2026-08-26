//! 影の欠落計器が返す型付きエラー。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//!
//! xtaskは外部のクレートへ依存しないため、thiserrorを使わず手書きのenumと`Display`で書く。
//! 様式は`acceptance/error.rs`と`depth_prepass_cost/error.rs`に倣う。
//!
//! 検収の器の破れと判定の破れを枝で内包するのは、この入口が起動と読み取りと判定を一続きに通すためである。
//! 3つを別の型で返すと、通す側が段ごとに変換を書くことになる。
//!
//! 引数の破れとアセットの生成と差分画像の書き出しをここが持つのは、どれも判定でも報告の読み取りでもないためである。
//! 人が読む1文へ写すのは`display`が持つ。

mod display;

use std::path::PathBuf;

use super::argument_error::影の欠落計器の引数の破れ;
use crate::acceptance::{判定の破れ, 検収エラー};

#[derive(Debug)]
pub(super) enum 影の欠落計器のエラー {
    検収の器が破れた(検収エラー),
    判定が破れた(判定の破れ),
    引数が破れた(影の欠落計器の引数の破れ),
    検証用アセットを生成できなかった,
    前の実行の差分画像を消せなかった { パス: PathBuf, 誤り: std::io::Error }, // 残ったままだと、書き出しに至らなかった実行の裁定材料に見える
    差分画像を書けなかった { パス: PathBuf, 誤り: std::io::Error },
    差分画像の寸法を書けなかった { パス: PathBuf, 誤り: std::io::Error },
}

impl std::error::Error for 影の欠落計器のエラー {}

impl From<検収エラー> for 影の欠落計器のエラー {
    fn from(破れ: 検収エラー) -> Self {
        Self::検収の器が破れた(破れ)
    }
}

impl From<判定の破れ> for 影の欠落計器のエラー {
    fn from(破れ: 判定の破れ) -> Self {
        Self::判定が破れた(破れ)
    }
}

impl From<影の欠落計器の引数の破れ> for 影の欠落計器のエラー {
    fn from(破れ: 影の欠落計器の引数の破れ) -> Self {
        Self::引数が破れた(破れ)
    }
}
