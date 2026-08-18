//! コマンド一覧の1件を表す値オブジェクト。所有するのは、既存の説明文からASCIIコマンド名と
//! メニュー表示用の要約を取り出す責務である。日本語のコマンド名だけをこの型が追加で持つ。

/// コマンド一覧の1件。`説明文`は"  ascii名 引数の構文  日本語の説明"の形式で統一されており、
/// ASCIIコマンド名は先頭の空白区切りの語から取り出せる。全文をこの1つのフィールドへ持つのは、
/// `cargo xtask`の一覧表示とメニューの使い方表示の両方が、同じ全文をそのまま必要とするためである。
/// 2フィールドとも`pub(super)`に留め、command_catalogの外からは以下のメソッド経由でのみ読ませる
/// (この形式の不変条件はcommand_catalogの結合テストが全件について検査して守る)。
#[derive(Clone, Copy)]
pub(crate) struct コマンド項目 {
    pub(super) 日本語名: &'static str,
    pub(super) 説明文: &'static str,
}

impl コマンド項目 {
    /// 日本語のコマンド名を返す。
    pub(crate) fn 日本語名(&self) -> &'static str {
        self.日本語名
    }

    /// 説明文の先頭語をASCIIコマンド名として取り出す。全項目が"  ascii名 ..."の形式で
    /// 統一されているという前提に立つ(この前提はcommand_catalogの結合テストが全件検査で守る)。
    pub(crate) fn ascii名(&self) -> &'static str {
        self.説明文.split_whitespace().next().unwrap_or("")
    }

    /// `cargo xtask`の引数なし一覧表示に載せる説明文そのもの(先頭の空白による字下げを含む)。
    pub(crate) fn 全文(&self) -> &'static str {
        self.説明文
    }

    /// メニューが「使い方: cargo xtask 」に続けて表示する行。先頭の字下げだけを取り除いた説明文である。
    pub(crate) fn 使い方の行(&self) -> &'static str {
        self.説明文.trim_start()
    }

    /// メニューの一覧行に載せる要約。先頭のASCIIコマンド名を取り除いた残り(引数の構文と日本語の説明)を
    /// 文字数で切り詰める。ASCIIコマンド名は一覧行の別の場所へ既に表示するため、ここでは重複させない。
    pub(crate) fn 要約(&self, 上限文字数: usize) -> String {
        let ascii名 = self.ascii名();
        let 先頭の空白を除いた説明文 = self.説明文.trim_start();
        let 残り = 先頭の空白を除いた説明文
            .strip_prefix(ascii名)
            .unwrap_or(先頭の空白を除いた説明文)
            .trim_start();
        let 全文字数 = 残り.chars().count();
        if 全文字数 <= 上限文字数 {
            return 残り.to_string();
        }
        let 先頭: String = 残り.chars().take(上限文字数).collect();
        format!("{先頭}…")
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 先頭語をascii名として取り出す() {
        let 項目 = コマンド項目 {
            日本語名: "検証の標準列",
            説明文: "  verify           検証の標準列を実行する",
        };
        assert_eq!(項目.ascii名(), "verify");
    }

    #[test]
    fn 短い説明はそのまま返す() {
        let 項目 = コマンド項目 {
            日本語名: "起動スモーク確認",
            説明文: "  smoke  短い説明",
        };
        assert_eq!(項目.要約(20), "短い説明");
    }

    #[test]
    fn 長い説明は切り詰めて省略記号を付ける() {
        let 項目 = コマンド項目 {
            日本語名: "テスト",
            説明文: "  cmd  あいうえおかきくけこ",
        };
        assert_eq!(項目.要約(5), "あいうえお…");
    }
}
