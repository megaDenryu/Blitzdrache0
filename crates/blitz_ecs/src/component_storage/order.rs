//! 個体構成要素の置き場の、実行時個体IDの昇順での走査と、その順序を戻す並べ直し。
//! 本体(`mod.rs`)から分けているのは、走査が反映の境界の外で毎刻み呼ばれ、並べ直しが反映の境界でだけ呼ばれるという
//! 呼び出し局面の違いによる(切り出しの根拠義務の条2)。不変条件「並びの印が偽なら密な列は実行時個体IDの昇順である」は索引が持ち、
//! 走査はその確認を索引へ頼む。

use super::個体構成要素の置き場;
use crate::component_marker::個体構成要素;
use crate::runtime_entity_id::ゲーム世界の実行時個体ID;

impl<T: 個体構成要素> 個体構成要素の置き場<T> {
    /// 持っている全個体を実行時個体IDの昇順で走査する。並びが崩れたまま呼ぶのはバグであり panic で止める。
    #[allow(non_snake_case)] // 日本語の識別子に含むASCIIの大文字はRustの命名規則の検査に掛かる。ユビキタス言語の登録語をそのまま使う。
    pub(crate) fn IDの昇順で走査する(&self) -> impl Iterator<Item = (ゲーム世界の実行時個体ID, &T)> {
        self.索引.並びが崩れていないことを確かめる(std::any::type_name::<T>());
        self.索引.密な個体の列().iter().copied().zip(self.密な値の列.iter())
    }

    /// 持っている全個体を実行時個体IDの昇順で走査し、値を可変に貸す。
    #[allow(non_snake_case)] // 日本語の識別子に含むASCIIの大文字はRustの命名規則の検査に掛かる。ユビキタス言語の登録語をそのまま使う。
    pub(crate) fn IDの昇順で可変に走査する(&mut self) -> impl Iterator<Item = (ゲーム世界の実行時個体ID, &mut T)> {
        self.索引.並びが崩れていないことを確かめる(std::any::type_name::<T>());
        self.索引.密な個体の列().iter().copied().zip(self.密な値の列.iter_mut())
    }

    /// 追加と取り除きで崩れた密な列の並びを、実行時個体IDの昇順へ戻す(契約25)。崩れていなければ何もしない。
    pub(crate) fn 密な列を添字の昇順へ並べ直す(&mut self) {
        if let Some(順列) = self.索引.添字の昇順へ並べ直す() {
            順列.適用する(&mut self.密な値の列);
        }
    }
}
