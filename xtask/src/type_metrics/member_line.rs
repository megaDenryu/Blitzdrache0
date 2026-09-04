//! 型本体の1行が、構造体のフィールド宣言か、列挙の枝の開始か、メソッド定義かを判定する。

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

/// 枝の名前で始まる行を枝の開始とみなす。呼び出し元が列挙本体の直下の行だけを渡すため、枝が持つ
/// 名前付きフィールドはここへ来ない。属性・注釈・複数行にまたがる属性の中身・閉じ括弧は、
/// どれも識別子で始まらないことで外れる。
pub fn 列挙の枝の開始か(行: &str) -> bool {
    行.trim_start().chars().next().is_some_and(|文字| 文字.is_alphabetic() || 文字 == '_')
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 修飾子付きのメソッドを判定する() {
        assert!(メソッド定義か("    pub fn 描画する(&self) {"));
        assert!(!メソッド定義か(" // fn 説明だけ"));
    }

    #[test]
    fn コロン区切りの宣言をフィールドとみなす() {
        assert!(フィールド定義か("    pub パス: PathBuf,"));
        assert!(フィールド定義か("    深さ: usize,"));
    }

    #[test]
    fn 属性とコメントと空行は数えない() {
        assert!(!フィールド定義か("    #[derive(Debug)]"));
        assert!(!フィールド定義か(" /// 説明: 何か"));
        assert!(!フィールド定義か(""));
    }

    #[test]
    fn 枝の名前で始まる行だけを枝の開始とみなす() {
        assert!(列挙の枝の開始か("    値なし,"));
        assert!(列挙の枝の開始か("    範囲外(u32),"));
        assert!(列挙の枝の開始か("    長さ不一致 { 期待: usize, 実際: usize },"));
        assert!(!列挙の枝の開始か("    #[error(\"説明\")]"));
        assert!(!列挙の枝の開始か("    /// 説明"));
        assert!(!列挙の枝の開始か("    )]"));
        assert!(!列挙の枝の開始か("    },"));
        assert!(!列挙の枝の開始か(""));
    }
}
