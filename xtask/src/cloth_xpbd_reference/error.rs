//! 布のXPBD参照比較の検収が返す型付きエラー。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//! xtaskは外部のクレートへ依存しないため、thiserrorを使わず手書きのenumと`Display`で書く(様式は`cloth_empty/error.rs`に倣う)。
//! 人が読む1文へ写すのは`display`が持つ。

mod display;

use crate::acceptance::{判定の破れ, 検収エラー};

#[derive(Debug)]
pub(super) enum 布のXPBD参照比較の検収エラー {
    検収の器が破れた(検収エラー),
    判定が破れた(判定の破れ),
    検証用アセットを生成できなかった,
    シェーダーの一時コピーを作れなかった(crate::shader_copy::シェーダーの一時コピーの破れ),
}

impl std::error::Error for 布のXPBD参照比較の検収エラー {}

impl From<検収エラー> for 布のXPBD参照比較の検収エラー {
    fn from(破れ: 検収エラー) -> Self {
        Self::検収の器が破れた(破れ)
    }
}

impl From<判定の破れ> for 布のXPBD参照比較の検収エラー {
    fn from(破れ: 判定の破れ) -> Self {
        Self::判定が破れた(破れ)
    }
}
