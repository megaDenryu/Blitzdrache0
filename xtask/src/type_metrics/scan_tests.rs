//! 1ファイルの走査が何を観測にするかの試験。構造体のフィールドとimplのメソッドの数え方、列挙の枝の数え方、
//! 入れ子の宣言を数えないことの3つを固定する。
#![allow(clippy::unwrap_used)]

use super::declaration_amount::宣言の分量;
use super::observation::観測;
use super::scan::走査する;

const 例: &str = "pub struct 台帳 {\n    件数: usize,\n}\nimpl 台帳 {\n    pub fn 新規() -> Self {\n        Self { 件数: 0 }\n    }\n    fn 数える(&self) -> usize {\n        self.件数\n    }\n}\n";

const 列挙の例: &str = "pub enum 破れ {
    #[error(\"長さ{実際}が期待{期待}と一致しない\")]
    長さ不一致 { 期待: usize, 実際: usize },
    範囲外(u32),
    位置ずれ {
        番号: usize,
        差: i32,
    },
}
";

#[test]
fn 構造体のフィールドとimplのメソッドを数える() {
    let 観測一覧 = 走査する(例);
    assert!(matches!(
        観測一覧[0],
        観測::型定義 {
            分量: 宣言の分量::構造体のフィールド数(1),
            ..
        }
    ));
    assert!(matches!(&観測一覧[1], 観測::実装ブロック { 自己型の経路, メソッド数: 2 } if 自己型の経路.型名() == "台帳"));
}

#[test]
fn 列挙は枝の数を数え枝の中のフィールドは数えない() {
    let 観測一覧 = 走査する(列挙の例);
    assert!(matches!(
        &観測一覧[0],
        観測::型定義 {
            型名,
            分量: 宣言の分量::列挙の枝数(3),
        } if 型名 == "破れ"
    ));
}

#[test]
fn 入れ子の宣言は数えない() {
    assert!(走査する("fn 外側() {\n    struct 内側 {\n        値: u32,\n    }\n}\n").is_empty());
}
