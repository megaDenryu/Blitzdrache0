//! 点光源の影の検収が返す型付きエラー。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//!
//! xtaskは外部のクレートへ依存しないため、thiserrorを使わず手書きのenumと`Display`で書く。
//! 様式は`acceptance/error.rs`と`depth_prepass_cost/error.rs`に倣う。
//!
//! 検収の器の破れと判定の破れを枝で内包するのは、この入口が起動と読み取りと判定を一続きに通すためである。
//! 3つを別の型で返すと、通す側が段ごとに変換を書くことになる。
//!
//! 検収世界の用意と計測用の構築をここが持つのは、どちらも判定でも報告の読み取りでもないためである。
//! 人が読む1文へ写すのは`display`が持つ。

mod display;

use crate::acceptance::{判定の破れ, 検収エラー};
use crate::release_build::計測用の構築の破れ;
use crate::world_setup::検収世界の用意の破れ;

#[derive(Debug)]
pub(super) enum 点光源の影の検収エラー {
    検収の器が破れた(検収エラー),
    判定が破れた(判定の破れ),
    検収世界を用意できなかった(検収世界の用意の破れ),
    計測用の構築が失敗した(計測用の構築の破れ),
}

impl std::error::Error for 点光源の影の検収エラー {}

impl From<検収エラー> for 点光源の影の検収エラー {
    fn from(破れ: 検収エラー) -> Self {
        Self::検収の器が破れた(破れ)
    }
}

impl From<判定の破れ> for 点光源の影の検収エラー {
    fn from(破れ: 判定の破れ) -> Self {
        Self::判定が破れた(破れ)
    }
}
