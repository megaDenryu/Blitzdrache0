//! 検収が見つけた、トレイトの型引数と境界にマーカーの型の名前を書いたふつうの実装の偽陽性の試験。反例はどれも rustc 1.94.0・clippy・rustfmt を通る。
//! 見出しの全体を照らしていたとき、`impl IndexMut<地点> for 地図`・`impl Extend<地点> for 経路`・`impl AddAssign<距離> for 走行の合計` が、マーカーを名乗る値オブジェクトを書き換えないのに違反になった。
//! いまはトレイトの型引数を照らさない。その実装は別の型のAPIであり、マーカーの型を引数に取る自由関数と同じ範囲にあるためである。
//! 境界にだけマーカーの型を書いた `impl<T: Into<地点>> 記録器<T>` は、境界で当たった実装が値で受ける `self` の関数も数えるため違反になり、台帳の区分 `境界で指した名前` で除く。

use super::super::name_match_exclusion_ledger::{台帳の行, 名前が当たった別の型の実装の台帳};
use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧, 台帳を与えた自己変更の違反の説明一覧};

const 地点で引く可変の添字: &str = "use std::ops::{Index, IndexMut};\npub trait M不変データ {}\n#[derive(Clone, Copy)]\npub struct 地点(pub usize);\nimpl M不変データ for 地点 {}\npub struct 地図(pub Vec<u8>);\nimpl Index<地点> for 地図 {\n    type Output = u8;\n    fn index(&self, 位置: 地点) -> &u8 {\n        &self.0[位置.0]\n    }\n}\nimpl IndexMut<地点> for 地図 {\n    fn index_mut(&mut self, 位置: 地点) -> &mut u8 {\n        &mut self.0[位置.0]\n    }\n}\n";
const 地点を足す経路: &str = "pub trait M不変データ {}\n#[derive(Clone, Copy)]\npub struct 地点(pub usize);\nimpl M不変データ for 地点 {}\npub struct 経路(pub Vec<地点>);\nimpl Extend<地点> for 経路 {\n    fn extend<I: IntoIterator<Item = 地点>>(&mut self, 列: I) {\n        self.0.extend(列);\n    }\n}\n";
const 距離を足す合計: &str = "use std::ops::AddAssign;\npub trait M不変データ {}\n#[derive(Clone, Copy)]\npub struct 距離(pub u32);\nimpl M不変データ for 距離 {}\npub struct 走行の合計(pub u32);\nimpl AddAssign<距離> for 走行の合計 {\n    fn add_assign(&mut self, 加える: 距離) {\n        self.0 += 加える.0;\n    }\n}\n";
const 地点へ変えられる値を積む記録器: &str = "pub trait M不変データ {}\n#[derive(Clone, Copy)]\npub struct 地点(pub usize);\nimpl M不変データ for 地点 {}\npub struct 記録器<T>(pub Vec<T>);\nimpl<T: Into<地点>> 記録器<T> {\n    pub fn 積む(&mut self, 値: T) {\n        self.0.push(値);\n    }\n}\n";

#[test]
fn トレイトの型引数にだけマーカーの型を書いた実装は違反にしない() {
    for 本文 in [地点で引く可変の添字, 地点を足す経路, 距離を足す合計] {
        let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 本文)]);
        assert!(説明一覧.is_empty(), "{説明一覧:?}");
    }
}

#[test]
fn 境界にだけマーカーの型を書いた実装は違反になり区分境界で指した名前の台帳の行で除ける() {
    let ソース一覧 = || vec![ソース("crates/a/src/x.rs", 地点へ変えられる値を積む記録器)];
    let 空の台帳 = 名前が当たった別の型の実装の台帳::行一覧から組む(Vec::new());
    let 説明一覧 = 台帳を与えた自己変更の違反の説明一覧(ソース一覧(), &空の台帳);
    assert!(説明一覧.iter().any(|説明| 説明.contains("型引数の並びの境界に名前 `地点` を識別子として含み")), "{説明一覧:?}");
    let 台帳 = 名前が当たった別の型の実装の台帳::行一覧から組む(vec![台帳の行 {
        マーカーの名前: "地点",
        パス: "crates/a/src/x.rs",
        見出し: "impl<T: Into<地点>> 記録器<T>",
        区分: "境界で指した名前",
        除外する理由: "境界の Into<地点> は地点の値を作るだけであり、地点の値への可変の参照も可変の借用も与えない",
    }]);
    assert!(台帳を与えた自己変更の違反の説明一覧(ソース一覧(), &台帳).is_empty());
}
