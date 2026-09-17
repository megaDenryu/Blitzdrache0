//! 型を消した置き場: 個体群が型ごとの置き場を1つの列で持つための、型引数を持たない口。
//! 具体の置き場への復元は `std::any::Any` で行い、型の識別を鍵にした索引がどの列の添字にどの型があるかを知る。
//! この口が持つのは、個体群が型を知らずに行う操作(個体が消えるときの取り除き・件数と占有量・並べ直し)だけである。
//! 参照: `_doc/設計/ゲーム世界の個体群の基盤.md`「判断5」。

use std::any::Any;

use crate::component_count::個体構成要素の型ごとの件数と占有量;
use crate::component_marker::個体構成要素;
use crate::component_storage::個体構成要素の置き場;
use crate::runtime_entity_id::ゲーム世界の実行時個体ID;

/// 型を消したまま個体の値を取り除いたときの、取り除けたかどうか。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 型を消した取り除きの結果 {
    取り除いた,
    持っていなかった,
}

pub(crate) trait 型を消した置き場: Send + Sync {
    /// 個体が持つこの型の値を取り除く。持っていなければ何もしない。
    fn 個体の値を取り除く(&mut self, 個体: ゲーム世界の実行時個体ID) -> 型を消した取り除きの結果;
    fn 型ごとの件数と占有量(&self) -> 個体構成要素の型ごとの件数と占有量;
    fn 密な列を添字の昇順へ並べ直す(&mut self);
    fn 型を消して参照する(&self) -> &dyn Any;
    fn 型を消して可変に参照する(&mut self) -> &mut dyn Any;
}

impl<T: 個体構成要素> 型を消した置き場 for 個体構成要素の置き場<T> {
    fn 個体の値を取り除く(&mut self, 個体: ゲーム世界の実行時個体ID) -> 型を消した取り除きの結果 {
        match self.取り除く(個体) {
            Some(_取り除いた値) => 型を消した取り除きの結果::取り除いた,
            None => 型を消した取り除きの結果::持っていなかった,
        }
    }

    fn 型ごとの件数と占有量(&self) -> 個体構成要素の型ごとの件数と占有量 {
        個体構成要素の型ごとの件数と占有量::生成する(std::any::type_name::<T>(), self.件数(), self.占有量のバイト数())
    }

    fn 密な列を添字の昇順へ並べ直す(&mut self) {
        個体構成要素の置き場::密な列を添字の昇順へ並べ直す(self);
    }

    fn 型を消して参照する(&self) -> &dyn Any {
        self
    }

    fn 型を消して可変に参照する(&mut self) -> &mut dyn Any {
        self
    }
}
