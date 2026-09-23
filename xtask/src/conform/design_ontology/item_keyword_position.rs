//! 項目の予約語(`impl`・`type`・`use`)が行の頭に在るかを読む走査。受け取るのはコードだけの1行、返すのはその行の中の行の頭でない項目の予約語の一覧である。
//! 検査器が宣言として読むのは行の頭の3つだけである(`impl_header.rs`・`type_alias_scan.rs`・`use_resolution.rs`)。行の途中に書いた宣言はどの読み口にも当たらず、違反にも件数にもならずに落ちる。
//! そのため検査器は、予約語ごとに項目を始める位置の条件を推し量ることをやめ、「項目の予約語は行の頭に書く」を1つの規約にし、行の頭でない現れを違反にする。
//! 行の頭とは、字下げと属性と可視性(`impl` では可視性の代わりに `unsafe`)を読み飛ばした位置のことであり、3つの読み口が読み始める位置と同じである。
//! 例外は項目を始めない3つの現れだけである。型の位置の `impl`(直前が `->`・`:`・`&`・`<`・`mut`)、精密な捕捉の `use<'a>`(直後が `<`)、生の識別子(`r#type`)である。
//! 属性の中の語(`#[cfg_attr(feature = "typescript", ts(type = "string"))]` の `type`)も宣言でないため数えない。走査範囲(`crates` 配下の `src`)の現状は0件である。

use super::attribute_span::属性が覆う範囲の走査;
use super::declaration_prefix::{先頭の属性を読み飛ばす, 属性と可視性を読み飛ばす};
use super::identifier_boundary::識別子として現れる位置一覧;

/// 項目を始める3つの予約語。検査器はこの3つを行の頭でだけ読む。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 項目の予約語 {
    実装,
    型の別名,
    取り込み,
}

impl 項目の予約語 {
    const 一覧: [Self; 3] = [Self::実装, Self::型の別名, Self::取り込み];

    const fn 表記(self) -> &'static str {
        match self {
            Self::実装 => "impl",
            Self::型の別名 => "type",
            Self::取り込み => "use",
        }
    }

    /// 行の頭でない位置に書いたときの、読み切れない理由。違反の説明がそのまま読み手へ返す。
    pub const fn 行の頭でない理由(self) -> &'static str {
        match self {
            Self::実装 => "`impl` を行の頭でない位置に書いている。検査器が実装として読むのは行の頭の `impl` だけであり、行の途中の実装の本体の関数を黙って検査から落とすため、行を改めて行の頭に書く",
            Self::型の別名 => "`type` を行の頭でない位置に書いている。検査器が型の別名として読むのは行の頭の `type` だけであり、行の途中の別名が作る名前の辺を黙って検査から落とすため、行を改めて行の頭に書く",
            Self::取り込み => "`use` を行の頭でない位置に書いている。検査器が取り込みとして読むのは行の頭の `use` だけであり、行の途中の取り込みが持ち込む名前を黙って検査から落とすため、行を改めて行の頭に書く",
        }
    }

    // その行でこの予約語を書いてよい唯一のバイト位置。読み口が読み始める位置と同じ読み飛ばし方をする。
    fn 行の頭の位置(self, 行: &str) -> usize {
        let 残り = match self {
            Self::実装 => {
                let 残り = 先頭の属性を読み飛ばす(行);
                残り.strip_prefix("unsafe").filter(|後ろ| 後ろ.starts_with(char::is_whitespace)).map_or(残り, str::trim_start)
            }
            Self::型の別名 | Self::取り込み => 属性と可視性を読み飛ばす(行),
        };
        行.len().saturating_sub(残り.len())
    }

    // その位置の現れが項目を始めうるか。型の位置の `impl`・精密な捕捉の `use<..>`・生の識別子は項目を始めない。
    fn 項目を始めうる現れか(self, 行: &str, 位置: usize) -> bool {
        let 前 = 行.get(..位置).unwrap_or_default();
        let 後ろ = 行.get(位置 + self.表記().len()..).unwrap_or_default();
        if 前.ends_with("r#") {
            return false;
        }
        match self {
            Self::実装 => !型の位置の前置きか(前.trim_end()),
            Self::取り込み => !後ろ.trim_start().starts_with('<'),
            Self::型の別名 => true,
        }
    }
}

// `impl` の直前の表記が、そこを型の位置にする前置き(`->`・`:`・`&`・`<`・参照の `mut`)で終わるか。
fn 型の位置の前置きか(前: &str) -> bool {
    let 参照のmutか = 前.strip_suffix("mut").is_some_and(|その前| その前.chars().next_back().is_none_or(|文字| !(文字.is_alphanumeric() || 文字 == '_')));
    前.ends_with("->") || 前.ends_with(':') || 前.ends_with('&') || 前.ends_with('<') || 参照のmutか
}

/// 項目の予約語の位置を行ごとに読む走査。属性が覆う範囲を行をまたいで覚えるため、1つのファイルを先頭の行から順に読む。
#[derive(Default)]
pub struct 項目の予約語の走査 {
    属性: 属性が覆う範囲の走査,
}

impl 項目の予約語の走査 {
    /// 1行の中の、行の頭でない位置に書いた項目の予約語の一覧。並びは行の中の位置の順である。
    pub fn 行の頭でない項目の予約語一覧(&mut self, 行: &str) -> Vec<項目の予約語> {
        let 属性の範囲一覧 = self.属性.属性が覆う範囲を読む(行);
        let mut 該当一覧: Vec<(usize, 項目の予約語)> = Vec::new();
        for 予約語 in 項目の予約語::一覧 {
            let 頭の位置 = 予約語.行の頭の位置(行);
            該当一覧.extend(
                識別子として現れる位置一覧(行, 予約語.表記())
                    .into_iter()
                    .filter(|位置| *位置 != 頭の位置 && !属性の範囲一覧.iter().any(|範囲| 範囲.contains(位置)))
                    .filter(|位置| 予約語.項目を始めうる現れか(行, *位置))
                    .map(|位置| (位置, 予約語)),
            );
        }
        該当一覧.sort_by_key(|(位置, _)| *位置);
        該当一覧.into_iter().map(|(_, 予約語)| 予約語).collect()
    }
}
