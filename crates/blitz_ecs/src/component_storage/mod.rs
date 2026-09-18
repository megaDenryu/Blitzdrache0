//! 個体構成要素の置き場: 同じ型の個体構成要素を実行時個体IDに対応付けて保持する疎な集合。
//! スロットの添字から密な列の位置を引く疎な配列と密な列のIDの側は疎な集合の索引(`index.rs`)が持ち、
//! この型は索引が返す位置で同じ並びの値の列を持つ。格納の方式はこの型の内部の選択であり、個体構成要素の型はこれを知らない(契約10)。
//!
//! 注意: 密な列の並びは、構造変更の反映の境界の外では常に実行時個体IDの昇順である(契約25)。取り除きと追加で並びが
//! 崩れたことは索引が覚え、反映の境界が `密な列を添字の昇順へ並べ直す` を呼んで戻す。並びの崩れた状態での走査は
//! バグであり、走査はそれを panic で止める。
//! 参照: `_doc/設計/ゲーム世界の個体群の基盤.md`「判断3」。

mod dense_position;
mod error;
mod index;
mod order;
mod permutation;
#[cfg(test)]
mod tests;

use std::mem::size_of;

use crate::component_marker::個体構成要素;
use crate::runtime_entity_id::ゲーム世界の実行時個体ID;
pub use error::個体構成要素の置き場エラー;
use index::疎な集合の索引;

/// 型Tの個体構成要素を実行時個体IDに対応付けて持つ疎な集合。
pub struct 個体構成要素の置き場<T: 個体構成要素> {
    索引: 疎な集合の索引,
    密な値の列: Vec<T>,
}

impl<T: 個体構成要素> 個体構成要素の置き場<T> {
    pub(crate) fn 生成する() -> Self {
        Self {
            索引: 疎な集合の索引::生成する(),
            密な値の列: Vec::new(),
        }
    }

    /// 個体へ値を追加する。その個体が既にこの型を持っていれば拒む(契約11)。
    pub(crate) fn 追加する(&mut self, 個体: ゲーム世界の実行時個体ID, 値: T) -> Result<(), 個体構成要素の置き場エラー> {
        let Some(位置) = self.索引.追加する(個体) else {
            return Err(個体構成要素の置き場エラー::既に同じ型を持っている {
                個体, 型の名前: std::any::type_name::<T>()
            });
        };
        if 位置 != self.密な値の列.len() {
            panic!("索引が返した位置{位置}が値の列の末尾{}と違う(索引と値の列は同じ並びを保つ)", self.密な値の列.len())
        }
        self.密な値の列.push(値);
        Ok(())
    }

    pub(crate) fn 参照する(&self, 個体: ゲーム世界の実行時個体ID) -> Option<&T> {
        self.索引.密な列の位置を参照する(個体).map(|位置| &self.密な値の列[位置])
    }

    pub(crate) fn 可変に参照する(&mut self, 個体: ゲーム世界の実行時個体ID) -> Option<&mut T> {
        self.索引.密な列の位置を参照する(個体).map(|位置| &mut self.密な値の列[位置])
    }

    /// 個体の値を取り除いて返す。索引が密な列の末尾を空いた位置へ動かしたなら、値の列も同じ入れ替えをする。
    pub(crate) fn 取り除く(&mut self, 個体: ゲーム世界の実行時個体ID) -> Option<T> {
        let 位置 = self.索引.取り除く(個体)?;
        Some(self.密な値の列.swap_remove(位置))
    }

    pub(crate) fn 件数(&self) -> usize {
        self.密な値の列.len()
    }

    /// 疎な配列と密な列が確保しているバイト数。中身の件数でなく確保ぶんを数えるのは、常時占有する量を答えるためである。
    pub(crate) fn 占有量のバイト数(&self) -> usize {
        self.索引.占有量のバイト数() + self.密な値の列.capacity() * size_of::<T>()
    }
}
