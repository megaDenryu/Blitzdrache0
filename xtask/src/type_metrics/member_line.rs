//! 型本体の1行が、構造体のフィールド宣言かメソッド定義かを判定する。

use super::keyword::修飾子を取り除く;

pub fn メソッド定義か(行: &str) -> bool {
    let 整形 = 行.trim();
    !整形.starts_with("//") && 修飾子を取り除く(整形).starts_with("fn ")
}

/// 名前と型をコロンで区切る宣言をフィールドとみなす。属性・コメント・空行は数えない。
pub fn フィールド定義か(行: &str) -> bool {
    let 整形 = 行.trim();
    if 整形.is_empty() || 整形.starts_with("//") || 整形.starts_with("#[") {
        return false;
    }
    match 整形.split_once(':') {
        Some((名前部分, _)) => !名前部分.is_empty() && !名前部分.contains('('),
        None => false,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 修飾子付きのメソッドを判定する() {
        assert!(メソッド定義か("    pub fn 描画する(&self) {"));
        assert!(!メソッド定義か("    // fn 説明だけ"));
    }

    #[test]
    fn コロン区切りの宣言をフィールドとみなす() {
        assert!(フィールド定義か("    pub パス: PathBuf,"));
        assert!(フィールド定義か("    深さ: usize,"));
    }

    #[test]
    fn 属性とコメントと空行は数えない() {
        assert!(!フィールド定義か("    #[derive(Debug)]"));
        assert!(!フィールド定義か("    /// 説明: 何か"));
        assert!(!フィールド定義か(""));
    }
}
