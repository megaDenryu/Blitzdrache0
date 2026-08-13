//! エンジンが宣言し、レンダラーが実行する1フレームの組み立て方。段階の列と深度プリパスの方式の2つを持つ。
//! 段階の語彙は`stage`、深度プリパスの方式は`depth_prepass`、段階列の妥当性の検査は`validation`にある。

#[cfg(test)]
mod frame_composition_tests;

use thiserror::Error;

mod depth_prepass;
mod stage;
mod validation;

pub use depth_prepass::深度プリパス方式;
pub use stage::フレーム段階;
use validation::検証する;

const 段階数上限: usize = 9;

#[derive(Debug, Clone, Copy)]
pub struct フレーム構成 {
    段階一覧: [Option<フレーム段階>; 段階数上限],
    深度プリパス方式: 深度プリパス方式,
}

impl フレーム構成 {
    pub fn 生成する(段階一覧: &[フレーム段階]) -> Result<Self, フレーム構成エラー> {
        検証する(段階一覧)?;
        let mut 格納先 = [None; 段階数上限];
        for (添字, 段階) in 段階一覧.iter().copied().enumerate() {
            格納先[添字] = Some(段階);
        }
        Ok(Self {
            段階一覧: 格納先,
            深度プリパス方式: 深度プリパス方式::使い色は近いか同値で比べる,
        })
    }

    /// 深度プリパスの方式を据えた構成を返す。生成の既定が`使い色は近いか同値で比べる`であるのは、2026-08-08の
    /// オーナー裁定がこの方式を全世界の既定に定めたためである(参照: `_doc/設計/放射輝度問い合わせ階層.md`「既定をBへ切り替えた再裁定」)。
    #[must_use]
    pub const fn 深度プリパス方式を据えた(self, 方式: 深度プリパス方式) -> Self {
        Self {
            段階一覧: self.段階一覧,
            深度プリパス方式: 方式,
        }
    }

    pub const fn 深度プリパス方式(&self) -> 深度プリパス方式 {
        self.深度プリパス方式
    }

    pub fn 段階一覧(&self) -> impl Iterator<Item = フレーム段階> + '_ {
        self.段階一覧.iter().flatten().copied()
    }

    pub fn 含む(&self, 対象: フレーム段階) -> bool {
        self.段階一覧().any(|段階| 段階 == 対象)
    }
}

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum フレーム構成エラー {
    #[error("フレーム構成は1段階以上でなければならない")]
    空,
    #[error("フレーム構成は9段階以下でなければならない")]
    段階数超過,
    #[error("フレーム段階が重複しているか依存順序に反している")]
    重複または順序不正,
    #[error("現在のシーン描画には影段階が必要である")]
    影なし,
    #[error("フレーム構成にはシーン段階が必要である")]
    シーンなし,
}
