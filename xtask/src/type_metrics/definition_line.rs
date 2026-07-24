//! struct・enumの定義開始行から型名を読み取る。

use super::keyword::修飾子を取り除く;

const 型名の終端文字: [char; 6] = ['<', '(', '{', ' ', ';', ':'];

pub fn 型名を読み取る(行: &str) -> Option<String> {
    let 整形 = 行.trim();
    if 整形.starts_with("//") {
        return None;
    }
    let 本体 = 修飾子を取り除く(整形);
    let 残り = 本体.strip_prefix("struct ").or_else(|| 本体.strip_prefix("enum "))?;
    let 型名: String = 残り.trim_start().chars().take_while(|文字| !型名の終端文字.contains(文字)).collect();
    if 型名.is_empty() { None } else { Some(型名) }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 可視性付きの構造体を読み取る() {
        assert_eq!(型名を読み取る("pub struct レンダラー {").unwrap(), "レンダラー");
        assert_eq!(型名を読み取る("pub(crate) enum 観測 {").unwrap(), "観測");
    }

    #[test]
    fn ジェネリクスとタプルの境界で切る() {
        assert_eq!(型名を読み取る("struct 台帳<T> {").unwrap(), "台帳");
        assert_eq!(型名を読み取る("pub struct 秒(f32);").unwrap(), "秒");
    }

    #[test]
    fn コメント行と無関係な行は読み取らない() {
        assert!(型名を読み取る("// struct 偽物 {").is_none());
        assert!(型名を読み取る("let 値 = 1;").is_none());
    }
}
