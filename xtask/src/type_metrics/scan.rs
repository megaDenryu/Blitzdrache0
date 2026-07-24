//! 1ファイルを行順に走査し、波括弧の深さを追いながら型定義とimplブロックの観測を集める。
//! 深さ0で始まる宣言だけを対象とするため、関数やimplの内側に入れ子で定義された型は数えない。

use super::body_kind::本体種別;
use super::observation::観測;
use super::{definition_line, impl_line};

enum 走査状態 {
    外側,
    本体 { 種別: 本体種別, 型名: String, 件数: usize },
}

pub fn 走査する(内容: &str) -> Vec<観測> {
    let mut 観測一覧 = Vec::new();
    let mut 状態 = 走査状態::外側;
    let mut 深さ: usize = 0;
    for 行 in 内容.lines() {
        let 行前の深さ = 深さ;
        深さ = 次の深さ(深さ, 行);
        状態 = 一行を処理する(状態, 行, 行前の深さ, 深さ, &mut 観測一覧);
    }
    観測一覧
}

fn 次の深さ(現在: usize, 行: &str) -> usize {
    行.chars().fold(現在, |深さ, 文字| match 文字 {
        '{' => 深さ + 1,
        '}' => 深さ.saturating_sub(1),
        _ => 深さ,
    })
}

fn 一行を処理する(
    状態: 走査状態, 行: &str, 行前の深さ: usize, 行後の深さ: usize, 観測一覧: &mut Vec<観測>
) -> 走査状態 {
    let 走査状態::本体 { 種別, 型名, 件数 } = 状態 else {
        return 宣言の開始を判定する(行, 行前の深さ, 行後の深さ, 観測一覧);
    };
    let 次の件数 = 件数 + usize::from(行前の深さ == 1 && 種別.数える対象か(行));
    if 行後の深さ == 0 {
        観測一覧.push(種別.観測にする(型名, 次の件数));
        return 走査状態::外側;
    }
    走査状態::本体 {
        種別, 型名, 件数: 次の件数
    }
}

fn 宣言の開始を判定する(行: &str, 行前の深さ: usize, 行後の深さ: usize, 観測一覧: &mut Vec<観測>) -> 走査状態 {
    if 行前の深さ != 0 {
        return 走査状態::外側;
    }
    let 宣言 = definition_line::型名を読み取る(行)
        .map(|型名| (本体種別::構造体, 型名))
        .or_else(|| impl_line::実装対象の型名(行).map(|型名| (本体種別::実装, 型名)));
    let Some((種別, 型名)) = 宣言 else {
        return 走査状態::外側;
    };
    if 行後の深さ == 0 {
        観測一覧.push(種別.観測にする(型名, 0));
        return 走査状態::外側;
    }
    走査状態::本体 { 種別, 型名, 件数: 0 }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    const 例: &str = "pub struct 台帳 {\n    件数: usize,\n}\nimpl 台帳 {\n    pub fn 新規() -> Self {\n        Self { 件数: 0 }\n    }\n    fn 数える(&self) -> usize {\n        self.件数\n    }\n}\n";

    #[test]
    fn 構造体のフィールドとimplのメソッドを数える() {
        let 観測一覧 = 走査する(例);
        assert!(matches!(観測一覧[0], 観測::型定義 { フィールド数: 1, .. }));
        assert!(matches!(観測一覧[1], 観測::実装ブロック { メソッド数: 2, .. }));
        assert_eq!(観測一覧[1].型名(), "台帳");
    }

    #[test]
    fn 入れ子の宣言は数えない() {
        assert!(走査する("fn 外側() {\n    struct 内側 {\n        値: u32,\n    }\n}\n").is_empty());
    }
}
