//! 項目を始めうる予約語の種類と、字句の木の中でそれが現れた場所の区分。受け取るのは予約語の表記と直前の字句、返すのは種類と、項目を始めうるかである。
//! 検査器が宣言として読むのは、行の頭の `impl`・`type`・`use`・`macro_rules!` だけである(`impl_header.rs`・`type_alias_scan.rs`・`use_resolution.rs`・`macro_body.rs`)。
//! 字句の木の走査(`token_tree_scan.rs`)が項目を始めうる現れを数え、読み口の答えとの突き合わせ(`readable_form_assertion.rs`)が、読み口の読まなかった現れを違反にする。
//! トークン木の外で項目を始めうるのは、直前の字句が、文法の上で項目の直前に置ける字句の閉じた集合に入るときだけである。
//! 集合は、`impl` では `{`・`}`・`;`・`]`・`unsafe`・`default`、`type` と `use` と `macro_rules!` では `{`・`}`・`;`・`]`・`pub`・`pub(..)` を閉じる `)` である。
//! 型の位置の `impl`(`->`・`:`・`&`・`'a`・`<`・`,`・`=`・`(`・`[`・`mut` の後ろ)と精密な捕捉の `use<'a>`(`+` の後ろ)と属性の中の語(`(`・`,`・`=` の後ろ)は、この集合に入らないため数えない。

use super::preceding_token::直前の字句;

/// 項目を始める4つの予約語。検査器はこの4つを行の頭でだけ読む。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum 項目の予約語 {
    実装,
    型の別名,
    取り込み,
    マクロの定義,
}

/// 字句の木の中で、項目を始めうる予約語が現れた場所。違反の説明が、読み手の直し方をこの区分で書き分ける。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 予約語の現れた場所 {
    トークン木の外で項目を始めうる位置,
    トークン木の中,
    マクロのメタ変数の位置,
}

impl 項目の予約語 {
    /// 語が `impl`・`type`・`use` のどれかなら、その予約語。`macro_rules` は後ろに `!` と名前が続くときだけ項目を始めるため、字句の木の走査が別に判定する。
    pub fn 表記から読む(語: &str) -> Option<Self> {
        match 語 {
            "impl" => Some(Self::実装),
            "type" => Some(Self::型の別名),
            "use" => Some(Self::取り込み),
            _ => None,
        }
    }

    /// 違反の説明に書く予約語の表記。
    pub const fn 表記(self) -> &'static str {
        match self {
            Self::実装 => "impl",
            Self::型の別名 => "type",
            Self::取り込み => "use",
            Self::マクロの定義 => "macro_rules!",
        }
    }

    /// 読み口がこの予約語について読む宣言の形。違反の説明がそのまま読み手へ返す。
    pub const fn 読み口が読む形(self) -> &'static str {
        match self {
            Self::実装 => "行の頭の `impl` から本体を開く `{` までの見出し",
            Self::型の別名 => "行の頭の `type` から `;` までの宣言",
            Self::取り込み => "行の頭の `use ` から `;` までの文",
            Self::マクロの定義 => "行の頭の `macro_rules!` から本体を閉じる括弧まで",
        }
    }

    /// 読み口が読まなかった宣言について、検査から落ちるもの。違反の説明がそのまま読み手へ返す。
    pub const fn 読まないと落ちるもの(self) -> &'static str {
        match self {
            Self::実装 => "実装の本体の関数",
            Self::型の別名 => "別名が作る名前の辺",
            Self::取り込み => "取り込みが持ち込む名前",
            Self::マクロの定義 => "本体の中の実装と関数",
        }
    }

    /// トークン木の外で、この直前の字句の後ろのこの予約語が項目を始めうるか。
    pub fn 項目を始めうる直前か(self, 直前: 直前の字句) -> bool {
        match self {
            Self::実装 => matches!(直前, 直前の字句::項目の区切り | 直前の字句::予約語のunsafe | 直前の字句::予約語のdefault),
            Self::型の別名 | Self::取り込み | Self::マクロの定義 => matches!(直前, 直前の字句::項目の区切り | 直前の字句::予約語のpub | 直前の字句::可視性を閉じる丸括弧),
        }
    }
}
