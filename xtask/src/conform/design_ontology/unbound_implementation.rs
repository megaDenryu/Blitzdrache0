//! 対象の型を決められない実装(全称の実装とマクロの本体の中の実装)のうち、自己変更を与える関数を持つものを集める工程。
//! `自己変更の検査` の2つ目の問いであり、受け取るのは無し、返すのは見つけた実装の一覧である。触るのは全ソースとトレイトの宣言の索引だけである。
//! 全称の実装は境界を評価できず、マクロの中の実装は対象の型がメタ変数(`$型`)であるため、検査器はどちらも `M不変データ` の型へ結び付けられない。
//! そのため自分の型(全称の実装なら型引数、マクロの中ならメタ変数)への可変参照を持つ関数を見つけたら黙って通さず、台帳(`unbound_implementation_ledger.rs`)と突き合わせる。

use std::ops::RangeInclusive;
use std::path::PathBuf;

use super::super::violation::違反;
use super::function_signature::自己変更を問う対象;
use super::impl_header::implの見出しを読む;
use super::impl_syntax::実装の見出しの構文;
use super::line_matching::波括弧が閉じる行;
use super::mutable_impl_scan::自己変更の検査;
use super::read_implementation::読んだ実装;
use super::self_modification_evidence::自己変更の根拠;

/// 対象の型を決められず、自己変更を与える関数を持つ実装1件。見出しは本体を開く `{` の手前までを、空白の並びを1つに揃えた表記であり、台帳の行と照らす鍵になる。
pub struct 対象の型を決められない実装 {
    pub パス: PathBuf,
    pub 見出し: String,
    行番号: usize,
    根拠: 自己変更の根拠,
}

impl 対象の型を決められない実装 {
    pub fn 違反にする(&self) -> 違反 {
        let 説明 = format!(
            "設計オントロジー: 対象の型を決められない実装 `{}` {}。全称の実装とマクロの中の実装は検査器が M不変データ の型へ結び付けられないため違反にする。M不変データ の型に当たらないなら、その理由を台帳へ書く",
            self.見出し,
            self.根拠.説明()
        );
        違反::行単位(self.パス.clone(), self.行番号, 説明)
    }
}

impl 自己変更の検査<'_> {
    /// 走査範囲の全称の実装とマクロの本体の中の実装のうち、自己変更を与える関数を持つものの一覧。
    pub fn 対象の型を決められない自己変更の実装一覧(&self) -> Vec<対象の型を決められない実装> {
        self.ソース一覧.iter().flat_map(|(パス, 行一覧)| self.ファイルの中の対象の型を決められない実装一覧(読んだファイル { パス, 行一覧 })).collect()
    }

    fn ファイルの中の対象の型を決められない実装一覧(&self, ファイル: 読んだファイル) -> Vec<対象の型を決められない実装> {
        let マクロの本体一覧 = マクロの本体の行の範囲一覧(ファイル.行一覧);
        let mut 一覧 = Vec::new();
        for 開始 in 0..ファイル.行一覧.len() {
            let Some(見出し) = implの見出しを読む(ファイル.行一覧, 開始) else {
                continue;
            };
            let Some(構文) = 実装の見出しの構文::読む(&見出し.表記) else {
                continue;
            };
            if !構文.全称の実装か() && !マクロの本体一覧.iter().any(|範囲| 範囲.contains(&開始)) {
                continue;
            }
            let 対象 = 自己変更を問う対象 {
                型名: 構文.対象.名前(),
                可変参照を対象にするか: 構文.対象.可変参照か,
            };
            let 揃えた見出し = 見出しの空白を揃える(&見出し.表記);
            let 実装 = 読んだ実装 {
                パス: ファイル.パス,
                行一覧: ファイル.行一覧,
                行番号: 開始 + 1,
                見出し,
            };
            if let Some(根拠) = self.実装が与える自己変更の根拠(&実装, &構文.種類, &対象) {
                一覧.push(対象の型を決められない実装 {
                    パス: ファイル.パス.clone(),
                    見出し: 揃えた見出し,
                    行番号: 開始 + 1,
                    根拠,
                });
            }
        }
        一覧
    }
}

// 走査する1つのファイル。パスとコードだけの行の一覧の組である。
struct 読んだファイル<'a> {
    パス: &'a PathBuf,
    行一覧: &'a [String],
}

// `macro_rules!` の本体が占める行の範囲(0始まりの添字)の一覧。
fn マクロの本体の行の範囲一覧(行一覧: &[String]) -> Vec<RangeInclusive<usize>> {
    (0..行一覧.len()).filter(|開始| 行一覧[*開始].trim_start().starts_with("macro_rules!")).map(|開始| 開始..=波括弧が閉じる行(行一覧, 開始)).collect()
}

// 見出しの表記から本体を開く `{` を除き、空白の並びを1つに揃える。
fn 見出しの空白を揃える(表記: &str) -> String {
    let 表記 = 表記.trim_end();
    表記.strip_suffix('{').unwrap_or(表記).split_whitespace().collect::<Vec<_>>().join(" ")
}
