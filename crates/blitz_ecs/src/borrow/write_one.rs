//! 一型を可変に借りる借り: 型Tの置き場への可変の参照を持ち、実行時個体IDの昇順の走査と個別の読み書きを提供する。
//! 参照: `_doc/設計/ゲーム世界の個体群の基盤.md`「判断6」。

use crate::component_marker::個体構成要素;
use crate::component_storage::個体構成要素の置き場;
use crate::runtime_entity_id::ゲーム世界の実行時個体ID;

/// 型Tを持つ個体群を読み書きするための借り。
pub struct 一型を可変に借りる借り<'a, T: 個体構成要素> {
    置き場: &'a mut 個体構成要素の置き場<T>,
}

impl<'a, T: 個体構成要素> 一型を可変に借りる借り<'a, T> {
    pub(crate) fn 生成する(置き場: &'a mut 個体構成要素の置き場<T>) -> Self {
        Self { 置き場 }
    }

    /// 個体が持つ値を読む。持っていなければ不在を返す。
    pub fn 参照する(&self, 個体: ゲーム世界の実行時個体ID) -> Option<&T> {
        self.置き場.参照する(個体)
    }

    /// 個体が持つ値を可変に貸す。持っていなければ不在を返す。
    pub fn 可変に参照する(&mut self, 個体: ゲーム世界の実行時個体ID) -> Option<&mut T> {
        self.置き場.可変に参照する(個体)
    }

    /// 型Tを持つ全個体を実行時個体IDの昇順で走査する。
    #[allow(non_snake_case)] // 日本語の識別子に含むASCIIの大文字はRustの命名規則の検査に掛かる。ユビキタス言語の登録語をそのまま使う。
    pub fn IDの昇順で走査する(&self) -> impl Iterator<Item = (ゲーム世界の実行時個体ID, &T)> {
        self.置き場.IDの昇順で走査する()
    }

    /// 型Tを持つ全個体を実行時個体IDの昇順で走査し、値を可変に貸す。
    #[allow(non_snake_case)] // 日本語の識別子に含むASCIIの大文字はRustの命名規則の検査に掛かる。ユビキタス言語の登録語をそのまま使う。
    pub fn IDの昇順で可変に走査する(&mut self) -> impl Iterator<Item = (ゲーム世界の実行時個体ID, &mut T)> {
        self.置き場.IDの昇順で可変に走査する()
    }

    /// 型Tを持つ個体の数。
    pub fn 件数(&self) -> usize {
        self.置き場.件数()
    }
}
