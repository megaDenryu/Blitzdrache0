//! 型を消した個体構成要素の値: 追加の予約が運ぶ、型引数を持たない箱の中身。反映のときに自分の型の置き場へ自分を入れる。
//! 標識の`trait`とは別の`trait`であり、標識を実装した全ての型がこれを自動で得る(標識に全称の実装を書かないという契約8はそのまま守る)。

use crate::component_marker::個体構成要素;
use crate::component_storage::個体構成要素の置き場エラー;
use crate::runtime_entity_id::ゲーム世界の実行時個体ID;
use crate::storage_collection::個体構成要素の置き場の集まり;

pub(crate) trait 型を消した個体構成要素の値: Send + Sync {
    fn 型の名前(&self) -> &'static str;
    /// 自分の型の置き場(無ければ登録する)へ個体の値として入れる。既に持っていれば拒む。
    fn 置き場へ入れる(self: Box<Self>, 集まり: &mut 個体構成要素の置き場の集まり, 個体: ゲーム世界の実行時個体ID) -> Result<(), 個体構成要素の置き場エラー>;
}

impl<T: 個体構成要素> 型を消した個体構成要素の値 for T {
    fn 型の名前(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn 置き場へ入れる(self: Box<Self>, 集まり: &mut 個体構成要素の置き場の集まり, 個体: ゲーム世界の実行時個体ID) -> Result<(), 個体構成要素の置き場エラー> {
        集まり.登録して可変に参照する::<T>().追加する(個体, *self)
    }
}
