//! エンジンが宣言し、レンダラーが実行する1フレームの組み立て方。処理位置の列と深度プリパスの方式の2つを持つ。
//! 処理位置の語彙は`stage`、深度プリパスの方式は`depth_prepass`、処理位置の列の妥当性の検査は`validation`にある。

#[cfg(test)]
mod frame_composition_tests;

use thiserror::Error;

mod depth_prepass;
mod stage;
mod validation;

pub use depth_prepass::深度プリパス方式;
pub use stage::フレームの処理位置;
use validation::フレームの処理位置の列を検証する;

const フレームの処理位置の数の上限: usize = 9;

#[derive(Debug, Clone, Copy)]
pub struct フレーム構成 {
    処理位置の一覧: [Option<フレームの処理位置>; フレームの処理位置の数の上限],
    深度プリパス方式: 深度プリパス方式,
}

impl フレーム構成 {
    pub fn 生成する(処理位置の一覧: &[フレームの処理位置]) -> Result<Self, フレーム構成エラー> {
        フレームの処理位置の列を検証する(処理位置の一覧)?;
        let mut 格納先 = [None; フレームの処理位置の数の上限];
        for (添字, 処理位置) in 処理位置の一覧.iter().copied().enumerate() {
            格納先[添字] = Some(処理位置);
        }
        Ok(Self {
            処理位置の一覧: 格納先,
            深度プリパス方式: 深度プリパス方式::使い色は近いか同値で比べる,
        })
    }

    /// 深度プリパスの方式を据えた構成を返す。生成の既定が`使い色は近いか同値で比べる`であるのは、2026-08-08の
    /// オーナー裁定がこの方式を全世界の既定に定めたためである(参照: `_doc/設計/放射輝度問い合わせ階層.md`「既定をBへ切り替えた再裁定」)。
    #[must_use]
    pub const fn 深度プリパス方式を据えた(self, 方式: 深度プリパス方式) -> Self {
        Self {
            処理位置の一覧: self.処理位置の一覧,
            深度プリパス方式: 方式,
        }
    }

    pub const fn 深度プリパス方式(&self) -> 深度プリパス方式 {
        self.深度プリパス方式
    }

    pub fn 処理位置の一覧(&self) -> impl Iterator<Item = フレームの処理位置> + '_ {
        self.処理位置の一覧.iter().flatten().copied()
    }

    pub fn 含むか(&self, 対象: フレームの処理位置) -> bool {
        self.処理位置の一覧().any(|処理位置| 処理位置 == 対象)
    }
}

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum フレーム構成エラー {
    #[error("フレーム構成は処理位置を1つ以上持たなければならない")]
    空,
    #[error("フレーム構成が持てる処理位置は9つ以下である")]
    処理位置の数の超過,
    #[error("フレームの処理位置が重複しているか依存順序に反している")]
    重複または順序不正,
    #[error("現在のシーン描画には影の処理位置が必要である")]
    影なし,
    #[error("フレーム構成にはシーンの処理位置が必要である")]
    シーンなし,
}
