//! 一型を読む借り: 型Tの置き場への共有の参照を持ち、実行時個体IDの昇順の走査と個別の読みを提供する。
//! 登録していない型の借りは置き場を持たず、空の置き場と同じ答えを返す。読む借りは型を登録しないため `&ゲーム世界の個体群` から作れる。
//! 参照: `_doc/設計/ゲーム世界の個体群の基盤.md`「判断6」。

use crate::component_marker::個体構成要素;
use crate::component_storage::個体構成要素の置き場;
use crate::runtime_entity_id::ゲーム世界の実行時個体ID;

/// 型Tを持つ個体群を読むための借り。
pub struct 一型を読む借り<'a, T: 個体構成要素> {
    置き場: Option<&'a 個体構成要素の置き場<T>>, // 不在は、型を登録していないため空として読むことを表す
}

impl<'a, T: 個体構成要素> 一型を読む借り<'a, T> {
    pub(crate) fn 生成する(置き場: Option<&'a 個体構成要素の置き場<T>>) -> Self {
        Self { 置き場 }
    }

    /// 個体が持つ値を読む。持っていなければ不在を返す。
    pub fn 参照する(&self, 個体: ゲーム世界の実行時個体ID) -> Option<&'a T> {
        self.置き場?.参照する(個体)
    }

    /// 型Tを持つ全個体を実行時個体IDの昇順で走査する。
    #[allow(non_snake_case)] // 日本語の識別子に含むASCIIの大文字はRustの命名規則の検査に掛かる。ユビキタス言語の登録語をそのまま使う。
    pub fn IDの昇順で走査する(&self) -> impl Iterator<Item = (ゲーム世界の実行時個体ID, &'a T)> {
        self.置き場.into_iter().flat_map(個体構成要素の置き場::IDの昇順で走査する)
    }

    /// 型Tを持つ個体の数。
    pub fn 件数(&self) -> usize {
        self.置き場.map_or(0, 個体構成要素の置き場::件数)
    }
}
