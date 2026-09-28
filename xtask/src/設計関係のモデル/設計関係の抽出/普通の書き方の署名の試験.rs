//! 普通のRustの書き方で書いた関数の署名を、抽出器と定義の式が関係を落とさずに読むことを固定する回帰試験。
//! 道具の都合でドメインのコードの書き方を縛らないため、`where` 句の境界(関数・実装・トレイトの見出しの次の行以降へ折ったものを含む)・実装の見出しの型引数の境界・`use` で取り込んだモジュールを通したパス・
//! クロージャのトレイトと関数ポインタ・境界の無い型引数のどれを書いても、関係を落とした抽出の欠落が0件であることを見る。

#[path = "普通の書き方の署名の試験/関連型と標準のトレイトの実装の試験.rs"]
mod 関連型と標準のトレイトの実装の試験;

use super::抽出の成果と欠け::抽出した設計関係グラフと抽出の欠け;
use super::抽出の試験の段取り::{原文から設計関係グラフと抽出の欠けを組む, 関係の表記一覧};

const 試験のパス: &str = "crates/blitz_game/src/試験.rs";

const 形のパス: &str = "crates/blitz_game/src/形.rs";

const 形の原文: &str = "pub struct 地図 {\n    広さ: u8,\n}\n";

fn 抽出する(原文: &str) -> 抽出した設計関係グラフと抽出の欠け {
    原文から設計関係グラフと抽出の欠けを組む(&[(試験のパス, 原文), (形のパス, 形の原文)])
}

// 関係を落とした抽出の欠落が0件であることを確かめ、関係の表記一覧を返す。
fn 欠落無しで関係を読む(原文: &str) -> Vec<String> {
    let 結果 = 抽出する(原文);
    let 欠落 = 結果.関係を落とした抽出の欠落へ写す();
    assert_eq!(欠落.合計の件数(), 0, "{}", 欠落.説明());
    関係の表記一覧(&結果)
}

// 反証: `where` 句の境界を読まないと、`P` は境界の無い型引数と読まれ、関数が口のトレイトを引数に取る関係を落とす。
#[test]
fn where句の境界を型引数の境界として読む() {
    let 表記一覧 = 欠落無しで関係を読む("pub trait 口 {}\n\npub fn 使う<P>(口: &P) -> u8\nwhere\n    P: 口,\n{\n    0\n}\n");
    assert!(表記一覧.contains(&"blitz_game::試験::使う 引数に取る blitz_game::試験::口".to_string()), "{表記一覧:?}");
}

// 反証: 実装の見出しの型引数の境界と `where` 句を読まないと、本体の関数の `T` を境界のトレイトとして読めない。
#[test]
fn 実装の見出しの型引数の境界とwhere句を読む() {
    let 原文 = "pub trait 口 {}\n\npub trait 耳 {}\n\npub struct 包み<T, U> {\n    中: T,\n    横: U,\n}\n\nimpl<T: 口, U> 包み<T, U>\nwhere\n    U: 耳,\n{\n    pub fn 貸す(&self, 横: &U) -> &T {\n        &self.中\n    }\n}\n";
    let 表記一覧 = 欠落無しで関係を読む(原文);
    assert!(表記一覧.contains(&"blitz_game::試験::包み::貸す 返す blitz_game::試験::口".to_string()), "{表記一覧:?}");
    assert!(表記一覧.contains(&"blitz_game::試験::包み::貸す 引数に取る blitz_game::試験::耳".to_string()), "{表記一覧:?}");
}

// 反証: トレイトの見出しの行だけを読むと、rustfmt が次の行以降へ折った `where` 句の境界を落とし、`T` を境界の無い型引数として除外に数える。
// 上位トレイトの宣言の抽出は、`where` 句を持つトレイトの宣言を保証範囲の外として欠落に数える(`cargo xtask conform` がこの形をリポジトリに書かせない)。
// 関数の署名の側の欠落はその1件のほかに出ないことを見る。
#[test]
fn トレイトの見出しの次の行以降へ折ったwhere句の境界を読む() {
    let 結果 = 抽出する("pub trait 口 {}\n\npub trait 読み手<T>\nwhere\n    T: 口,\n{\n    fn 読む(&self, 値: &T);\n}\n");
    let 表記一覧 = 関係の表記一覧(&結果);
    assert!(表記一覧.contains(&"blitz_game::試験::読み手::読む 引数に取る blitz_game::試験::口".to_string()), "{表記一覧:?}");
    let 署名の側の行一覧: Vec<_> = 結果.抽出できなかった行一覧.iter().filter(|行| !行.理由.種別の呼び名().starts_with("トレイトの宣言")).collect();
    assert!(署名の側の行一覧.is_empty(), "{署名の側の行一覧:?}");
}

// 反証: `use std::fmt;` の後の `fmt::Result` を修飾の付いた読めない表記とすると、標準の `Display` の実装を普通に書くだけで欠落が出る。
#[test]
fn useで取り込んだ標準ライブラリのモジュールを通したパスは関係を作らない() {
    let 原文 = "use std::fmt;\n\npub struct 割合 {\n    値: u8,\n}\n\nimpl fmt::Display for 割合 {\n    fn fmt(&self, 書き先: &mut fmt::Formatter<'_>) -> fmt::Result {\n        todo!()\n    }\n}\n";
    let 表記一覧 = 欠落無しで関係を読む(原文);
    assert!(!表記一覧.iter().any(|表記| 表記.ends_with("Result") || 表記.ends_with("Formatter")), "{表記一覧:?}");
}

// 反証: 修飾の無い名前を定義の探索だけで解決すると、`use std::fmt::Display;` の後の `Display` と、取り込まずに書いたプレリュードの `Iterator` が、
// `std::fmt::Display` と書いたときと違ってモジュールパスの空の節点になる。
#[test]
fn useで標準ライブラリから取り込んだ名前とプレリュードのトレイトは修飾して書いたときと同じく関係を作らない() {
    let 原文 = "use std::fmt::Display;\n\npub fn 見せる(値: &dyn Display) -> impl Display {\n    todo!()\n}\n\npub fn 並べる() -> impl Iterator<Item = u8> {\n    todo!()\n}\n";
    let 結果 = 抽出する(原文);
    let 表記一覧 = 関係の表記一覧(&結果);
    assert!(!表記一覧.iter().any(|表記| 表記.ends_with("Display") || 表記.ends_with("Iterator")), "{表記一覧:?}");
    assert!(結果.抽出できなかった行一覧.is_empty(), "{:?}", 結果.抽出できなかった行一覧);
}

// 反証: 修飾を読まないと、`use crate::形;` の後の `形::地図` と `crate::形::地図` が地図の定義へたどり着かない。
#[test]
fn useで取り込んだクレートのモジュールと起点の予約語を通したパスは定義へたどり着く() {
    let 原文 = "use crate::形;\n\npub fn 読む(地図: &形::地図) -> crate::形::地図 {\n    todo!()\n}\n";
    let 表記一覧 = 欠落無しで関係を読む(原文);
    assert!(表記一覧.contains(&"blitz_game::試験::読む 引数に取る blitz_game::形::地図".to_string()), "{表記一覧:?}");
    assert!(表記一覧.contains(&"blitz_game::試験::読む 返す blitz_game::形::地図".to_string()), "{表記一覧:?}");
}

// 反証: 外部クレートの修飾を落とさずに読めないとすると、`blitz_math::変位<blitz_math::ワールド>` と `use` した `変位<ワールド>` が別の扱いになる。
#[test]
fn 外部クレートの修飾を付けたフレーム型は修飾の無い表記と同じ節点になる() {
    let 原文 = "use blitz_math::{ワールド, 変位};\n\npub fn 修飾して返す() -> blitz_math::変位<blitz_math::ワールド> {\n    todo!()\n}\n\npub fn 取り込んで返す() -> 変位<ワールド> {\n    todo!()\n}\n";
    let 表記一覧 = 欠落無しで関係を読む(原文);
    assert!(表記一覧.contains(&"blitz_game::試験::修飾して返す 返す 変位<ワールド>".to_string()), "{表記一覧:?}");
    assert!(表記一覧.contains(&"blitz_game::試験::取り込んで返す 返す 変位<ワールド>".to_string()), "{表記一覧:?}");
}

// 反証: クロージャのトレイトと関数ポインタを読めない表記とすると、関数を受け渡す関数を書くだけで欠落が出る。
#[test]
fn クロージャのトレイトと関数ポインタの引数と戻り値の型を読む() {
    let 原文 = "pub struct 地点 {\n    番号: u8,\n}\n\npub fn 選ぶ(判定: impl Fn(&地点) -> bool) {}\n\npub fn 変える(変換: fn(u8) -> 地点) {}\n";
    let 表記一覧 = 欠落無しで関係を読む(原文);
    assert!(表記一覧.contains(&"blitz_game::試験::選ぶ 引数に取る blitz_game::試験::地点".to_string()), "{表記一覧:?}");
    assert!(表記一覧.contains(&"blitz_game::試験::変える 引数に取る blitz_game::試験::地点".to_string()), "{表記一覧:?}");
}

// 反証: 境界の無い型引数を関係を落とした欠落に数えると、総称の関数を書くだけで設計の検証が落ちる。境界の無い型引数はどの設計概念も名指さない。
#[test]
fn 境界の無い型引数は欠落に数えず明示して除外する() {
    let 結果 = 抽出する("pub fn 通す<T>(値: T) -> T {\n    値\n}\n");
    assert_eq!(結果.関係を落とした抽出の欠落へ写す().合計の件数(), 0);
    assert!(結果.抽出できなかった行一覧.iter().any(|行| 行.理由.種別の呼び名() == "関数の署名の型の表記が型引数である"), "{:?}", 結果.抽出できなかった行一覧);
}
