//! struct・enumの定義開始行から、宣言の種別と型名を読み取る。

use super::body_kind::本体種別;
use super::keyword::修飾子を取り除く;

const 型名の終端文字: [char; 6] = ['<', '(', '{', ' ', ';', ':'];

pub fn 宣言の種別と型名を読み取る(行: &str) -> Option<(本体種別, String)> {
    let 整形 = 行.trim();
    if 整形.starts_with("//") {
        return None;
    }
    let 本体 = 修飾子を取り除く(整形);
    let (種別, 残り) = match 本体.strip_prefix("struct ") {
        Some(残り) => (本体種別::構造体, 残り),
        None => (本体種別::列挙, 本体.strip_prefix("enum ")?),
    };
    let 型名: String = 残り.trim_start().chars().take_while(|文字| !型名の終端文字.contains(文字)).collect();
    if 型名.is_empty() { None } else { Some((種別, 型名)) }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn 型名(行: &str) -> Option<String> {
        宣言の種別と型名を読み取る(行).map(|(_, 型名)| 型名)
    }

    #[test]
    fn 可視性付きの構造体と列挙を読み取る() {
        assert_eq!(型名("pub struct レンダラー {").unwrap(), "レンダラー");
        assert_eq!(型名("pub(crate) enum 観測 {").unwrap(), "観測");
    }

    #[test]
    fn 構造体と列挙を別の種別として返す() {
        let (種別, _) = 宣言の種別と型名を読み取る("pub struct レンダラー {").unwrap();
        assert!(matches!(種別, 本体種別::構造体));
        let (種別, _) = 宣言の種別と型名を読み取る("pub enum 観測 {").unwrap();
        assert!(matches!(種別, 本体種別::列挙));
    }

    #[test]
    fn ジェネリクスとタプルの境界で切る() {
        assert_eq!(型名("struct 台帳<T> {").unwrap(), "台帳");
        assert_eq!(型名("pub struct 秒(f32);").unwrap(), "秒");
    }

    #[test]
    fn コメント行と無関係な行は読み取らない() {
        assert!(型名("// struct 偽物 {").is_none());
        assert!(型名("let 値 = 1;").is_none());
    }
}
