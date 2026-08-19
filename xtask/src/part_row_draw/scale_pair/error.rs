//! 件数を変えた並びの対の検収が返す型付きエラー。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//!
//! xtaskは外部のクレートへ依存しないため、thiserrorを使わず手書きのenumと`Display`で書く。
//! 様式は`acceptance/error.rs`と`village_draw/error.rs`に倣う。
//!
//! 検収の器の破れと判定の破れと世界の用意の破れを枝で内包するのは、この入口が焼き付けと起動と読み取りと判定を
//! 一続きに通すためである。段ごとに別の型で返すと、通す側が段ごとに変換を書くことになる。
//! 人が読む1文へ写すのは`display`が持つ。

mod display;

use crate::acceptance::{判定の破れ, 検収エラー};
use crate::asset_generator::生成器エラー;
use crate::world_setup::検収世界の用意の破れ;

#[derive(Debug)]
pub(in crate::part_row_draw) enum 部品で組んだ並びの検収エラー {
    検収の器が破れた(検収エラー),
    判定が破れた(判定の破れ),
    検収世界を用意できなかった(検収世界の用意の破れ),
    アセットを焼けなかった(生成器エラー),
    ソースアセットを生成できなかった,
    前回の焼き上がりを消せなかった(std::io::Error),
}

impl std::error::Error for 部品で組んだ並びの検収エラー {}

impl From<検収エラー> for 部品で組んだ並びの検収エラー {
    fn from(破れ: 検収エラー) -> Self {
        Self::検収の器が破れた(破れ)
    }
}

impl From<判定の破れ> for 部品で組んだ並びの検収エラー {
    fn from(破れ: 判定の破れ) -> Self {
        Self::判定が破れた(破れ)
    }
}

impl From<生成器エラー> for 部品で組んだ並びの検収エラー {
    fn from(破れ: 生成器エラー) -> Self {
        Self::アセットを焼けなかった(破れ)
    }
}
