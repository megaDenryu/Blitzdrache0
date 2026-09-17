//! 個体構成要素の置き場の集まり: 型を消した置き場の列と、型の識別から列の添字を引く索引。
//! 個体群がこの型を持ち、置き場の登録と具体の型への復元、および型を知らずに全置き場へ行う操作をここへ閉じる。
//!
//! 注意: 列の添字は、型を初めて登録したときに末尾へ足して以後変えない。索引がこれに依存するため、置き場を削除する操作は持たない。
//! 型を登録していない型への読みの問い合わせは、空の置き場(1件も持たない)と同じ答えになる。
//! 参照: `_doc/設計/ゲーム世界の個体群の基盤.md`「判断5」。

use std::any::TypeId;
use std::collections::HashMap;

use crate::component_count::個体構成要素の型ごとの件数と占有量;
use crate::component_marker::個体構成要素;
use crate::component_storage::個体構成要素の置き場;
use crate::erased_storage::型を消した置き場;
use crate::runtime_entity_id::ゲーム世界の実行時個体ID;

pub(crate) struct 個体構成要素の置き場の集まり {
    置き場一覧: Vec<Box<dyn 型を消した置き場>>,
    型から列の添字への索引: HashMap<TypeId, usize>,
}

impl 個体構成要素の置き場の集まり {
    pub(crate) fn 生成する() -> Self {
        Self {
            置き場一覧: Vec::new(),
            型から列の添字への索引: HashMap::new(),
        }
    }

    /// 型Tの置き場を読む。登録していない型には不在を返し、呼び出し側はそれを空の置き場として扱う。
    pub(crate) fn 参照する<T: 個体構成要素>(&self) -> Option<&個体構成要素の置き場<T>> {
        let 添字 = *self.型から列の添字への索引.get(&TypeId::of::<T>())?;
        Some(具体の型へ復元する::<T>(self.置き場一覧[添字].型を消して参照する()))
    }

    /// 型Tの置き場を可変に借りる。登録していなければ、空の置き場を末尾へ足してから借りる。
    pub(crate) fn 登録して可変に参照する<T: 個体構成要素>(&mut self) -> &mut 個体構成要素の置き場<T> {
        let 添字 = self.登録して列の添字を得る::<T>();
        具体の型へ可変に復元する::<T>(self.置き場一覧[添字].型を消して可変に参照する())
    }

    /// 型TとUの置き場を同時に可変に借りる。同じ型を2度可変に借りる要求は呼び出し側のコードの誤りであり panic で止める。
    pub(crate) fn 二つを登録して可変に参照する<T: 個体構成要素, U: 個体構成要素>(&mut self) -> (&mut 個体構成要素の置き場<T>, &mut 個体構成要素の置き場<U>) {
        let 甲の添字 = self.登録して列の添字を得る::<T>();
        let 乙の添字 = self.登録して列の添字を得る::<U>();
        let Ok([甲, 乙]) = self.置き場一覧.get_disjoint_mut([甲の添字, 乙の添字]) else {
            panic!("同じ型{}の置き場を2度可変に借りようとした(2型の借りは互いに異なる型を指す)", std::any::type_name::<T>())
        };
        (具体の型へ可変に復元する::<T>(甲.型を消して可変に参照する()), 具体の型へ可変に復元する::<U>(乙.型を消して可変に参照する()))
    }

    /// 型の識別で置き場を型を消したまま可変に借りる。登録していない型には不在を返す。
    pub(crate) fn 型を消したまま可変に参照する(&mut self, 型: TypeId) -> Option<&mut dyn 型を消した置き場> {
        let 添字 = *self.型から列の添字への索引.get(&型)?;
        Some(self.置き場一覧[添字].as_mut())
    }

    pub(crate) fn 個体の全個体構成要素を取り除く(&mut self, 個体: ゲーム世界の実行時個体ID) {
        for 置き場 in &mut self.置き場一覧 {
            置き場.個体の値を取り除く(個体);
        }
    }

    pub(crate) fn 全部の密な列を添字の昇順へ並べ直す(&mut self) {
        for 置き場 in &mut self.置き場一覧 {
            置き場.密な列を添字の昇順へ並べ直す();
        }
    }

    pub(crate) fn 型ごとの件数と占有量を一覧する(&self) -> Vec<個体構成要素の型ごとの件数と占有量> {
        self.置き場一覧.iter().map(|置き場| 置き場.型ごとの件数と占有量()).collect()
    }

    fn 登録して列の添字を得る<T: 個体構成要素>(&mut self) -> usize {
        if let Some(添字) = self.型から列の添字への索引.get(&TypeId::of::<T>()) {
            return *添字;
        }
        let 添字 = self.置き場一覧.len();
        self.置き場一覧.push(Box::new(個体構成要素の置き場::<T>::生成する()));
        self.型から列の添字への索引.insert(TypeId::of::<T>(), 添字);
        添字
    }
}

fn 具体の型へ復元する<T: 個体構成要素>(型を消した: &dyn std::any::Any) -> &個体構成要素の置き場<T> {
    let Some(置き場) = 型を消した.downcast_ref::<個体構成要素の置き場<T>>() else {
        panic!("索引が指す列の添字に型{}の置き場が無い(列の添字は登録時に決まり以後変わらない)", std::any::type_name::<T>())
    };
    置き場
}

fn 具体の型へ可変に復元する<T: 個体構成要素>(型を消した: &mut dyn std::any::Any) -> &mut 個体構成要素の置き場<T> {
    let Some(置き場) = 型を消した.downcast_mut::<個体構成要素の置き場<T>>() else {
        panic!("索引が指す列の添字に型{}の置き場が無い(列の添字は登録時に決まり以後変わらない)", std::any::type_name::<T>())
    };
    置き場
}
