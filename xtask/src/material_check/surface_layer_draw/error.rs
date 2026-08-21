//! 地表の層の重ね合わせの検収が返す型付きエラー。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//!
//! xtaskは外部のクレートへ依存しないため、thiserrorを使わず手書きのenumと`Display`で書く。
//! 様式は`acceptance/error.rs`と`multi_material_draw/error.rs`に倣う。
//!
//! 検収用の世界を書き出す破れをこの型が持つのは、この入口が判定の材料になる世界を自分で組み立てるためである。
//! 人が読む1文へ写すのは`display`が持つ。

mod display;

use blitz_asset_compiler::地表材質の重み格子エラー;

use crate::acceptance::{判定の破れ, 検収エラー};

#[derive(Debug)]
pub(super) enum 地表の層の検収エラー {
    検収の器が破れた(検収エラー),
    判定が破れた(判定の破れ),
    世界のソースを書き出せなかった(String),
    実行時アセットを焼けなかった,
}

impl std::error::Error for 地表の層の検収エラー {}

impl From<検収エラー> for 地表の層の検収エラー {
    fn from(破れ: 検収エラー) -> Self {
        Self::検収の器が破れた(破れ)
    }
}

impl From<判定の破れ> for 地表の層の検収エラー {
    fn from(破れ: 判定の破れ) -> Self {
        Self::判定が破れた(破れ)
    }
}

impl From<地表材質の重み格子エラー> for 地表の層の検収エラー {
    fn from(破れ: 地表材質の重み格子エラー) -> Self {
        Self::世界のソースを書き出せなかった(破れ.to_string())
    }
}
