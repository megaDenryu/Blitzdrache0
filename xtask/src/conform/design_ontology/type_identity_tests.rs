//! 同名の型が複数のクレートや同じクレートの別のモジュールにあるときの、定義と固有の `impl` の採り方の試験と、純粋データ規約を役割の実装からたどった定義に当てる試験。
//! 型の同一性は標準的なファイル配置から推定したモジュールパスに定義を囲む `mod 名 { … }` の並びを繋いだ定義の位置のモジュール + 型名であり、実装の位置のモジュールの直下の定義、無ければ `use` で取り込んだモジュールの直下の定義を採る。`#[path` の属性が正規形であればこの推定はそのまま成り立つ。

use super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

#[test]
fn 構文解析_別クレートに同名の型があるときは実装と同じファイルの定義を採る() {
    let 甲 = ソース("crates/a/src/x.rs", "pub struct 位置 {\n    pub 東: f32,\n}\nimpl M不変データ for 位置 {}\n");
    let 乙 = ソース("crates/b/src/y.rs", "pub struct 位置<'a> {\n    値: &'a f32,\n}\n");
    assert!(全部の説明関数を連ねた違反の説明一覧(vec![甲, 乙]).is_empty());
}

#[test]
fn 構文解析_同じファイルに定義が無く別のファイルに同名の定義が2つあれば違反にする() {
    let 甲 = ソース("crates/a/src/x.rs", "pub struct 位置 {\n    pub 東: f32,\n}\n");
    let 乙 = ソース("crates/b/src/y.rs", "pub struct 位置 {\n    pub 北: f32,\n}\n");
    let 丙 = ソース("crates/c/src/z.rs", "impl M不変データ for 位置 {}\n");
    let 期待 = "設計オントロジー: M不変データ `位置` の定義(struct/enum)が見つかりません".to_string();
    assert_eq!(全部の説明関数を連ねた違反の説明一覧(vec![甲, 乙, 丙]), vec![期待]);
}

#[test]
fn 構文解析_別クレートの同名の型の固有のimplの可変参照メソッドは違反にならない() {
    let 甲 = ソース(
        "crates/a/src/x.rs",
        "pub struct 規則;
impl M不変データ for 規則 {}
impl M規則 for 規則 {}
",
    );
    let 乙 = ソース(
        "crates/b/src/y.rs",
        "pub struct 規則;
impl 規則 {
    pub fn 変える(&mut self) {}
}
",
    );
    assert!(全部の説明関数を連ねた違反の説明一覧(vec![甲, 乙]).is_empty());
}

#[test]
fn 構文解析_内部可変性を持つ規則は違反になる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 規則 {
    速さ: std::cell::RefCell<f32>,
}
impl M不変データ for 規則 {}
impl M規則 for 規則 {}
",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M不変データ `規則` の定義は内部可変性 `RefCell` を持てません"));
}

#[test]
fn 構文解析_impl_mdto_forが無く役割の実装だけの型にも純粋データ規約を当てる() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub enum 意図<'a> {\n    向く(&'a f32),\n}\nimpl Mコマンド for 意図<'_> {}\npub struct 信号 {\n    値: std::cell::Cell<u8>,\n}\nimpl M入力 for 信号 {}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 2);
    assert!(説明一覧[0].contains("Mコマンド `意図` の定義は参照(&)を持てません"));
    assert!(説明一覧[1].contains("M入力 `信号` の定義は内部可変性 `Cell` を持てません"));
}

#[test]
fn 構文解析_同じ型が複数の役割を持っても純粋データ規約の違反は1回だけ報告する() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 位置 {\n    値: std::cell::RefCell<f32>,\n}\nimpl M不変データ for 位置 {}\nimpl M状態 for 位置 {}\nimpl M入力 for 位置 {}\n",
    )];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1);
    assert!(説明一覧[0].contains("M不変データ `位置` の定義は内部可変性 `RefCell` を持てません"));
}

#[test]
fn 構文解析_日本語モジュールの正規形のpath属性は型の同一性の推定を壊さない() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/lib.rs",
        "#[path = \"接触.rs\"]
mod 接触;
pub struct 位置 {
    pub 東: f32,
}
impl M不変データ for 位置 {}
",
    )];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}
