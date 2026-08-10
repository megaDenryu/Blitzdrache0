//! 公開の`ライティング入力`をレンダラー内部の分解形へ写した、1フレームの照明入力。
//! 直接光・間接照明・直接影・有効性の4つに分け、GPUへの梱包はこの分解形だけを受け取る
//! (参照: `_doc/設計/放射輝度問い合わせ階層.md`「現行ライティング入力の改鋳(段1で行う)」)。
//!
//! 公開入口を分解形へ写す変換はこのモジュールの`ライティング入力から写す`1箇所だけが行う。
//! 固定6フィールドの公開型を照明の一般契約の正本にしないため、GPU側の梱包は公開型を受け取らない。

mod direct_light;
mod direct_shadow;
mod indirect_light;
mod validity;

#[cfg(test)]
mod decomposition_tests;

use crate::lighting_input::ライティング入力;

pub(crate) use direct_light::直接光入力;
pub(crate) use direct_shadow::{直接光の可視性, 直接影入力};
pub(crate) use indirect_light::間接照明入力;
pub(crate) use validity::照明の有効性;

pub(crate) struct フレーム照明入力 {
    直接光: 直接光入力,
    間接照明: 間接照明入力,
    直接影: 直接影入力,
    有効性: 照明の有効性,
}

impl フレーム照明入力 {
    /// 公開入口から分解形への唯一の変換点。レンダラー境界で1フレームにつき1回だけ呼ぶ。
    pub(crate) fn ライティング入力から写す(入力: &ライティング入力) -> Self {
        Self {
            直接光: 直接光入力::固定 {
                方向光: 入力.方向光(),
                局所光源列: 入力.局所光源列(),
            },
            間接照明: 間接照明入力::定数近似(入力.環境光()),
            直接影: 直接影入力::生成する(入力.影を落とすか()),
            有効性: 照明の有効性::真偽から写す(入力.有効か()),
        }
    }

    pub(crate) fn 直接光(&self) -> &直接光入力 {
        &self.直接光
    }

    pub(crate) fn 間接照明(&self) -> &間接照明入力 {
        &self.間接照明
    }

    pub(crate) fn 直接影(&self) -> 直接影入力 {
        self.直接影
    }

    pub(crate) fn 有効性(&self) -> 照明の有効性 {
        self.有効性
    }
}
