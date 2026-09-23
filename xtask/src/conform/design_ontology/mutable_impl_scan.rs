//! 型の定義に属する固有・トレイトの実装から、可変参照メソッドを探す工程。
//! 対象の型の表記はパスの最後の要素で型名に照らし、パスの修飾があればそのモジュールパスを定義のモジュールパスと突き合わせる(`read_implementation.rs`)。
//! 対象が可変参照(`&mut X`・`&'a mut X`・`Pin<&mut X>`)のトレイトの実装は、関数を1つでも持てば可変とみなす。

use super::impl_header::implの見出しを読む;
use super::impl_syntax::実装の見出しの構文;
use super::read_implementation::読んだ実装;
use super::syntax_checker::クレート構文検査;
use super::trait_implementation::トレイト実装型;
use super::type_definition::定義ブロックの結果;

impl クレート構文検査 {
    /// その型に属する固有・トレイトの実装に `&mut self` があるか。定義に属する実装だけを見て、別のモジュールの同名の型を混ぜない。
    pub fn 可変参照メソッドを含むか(&self, 型: &トレイト実装型) -> bool {
        let 定義ブロックの結果::見つかった { パス: 定義のパス, .. } = self.型の定義ブロック(型) else {
            return false;
        };
        self.ソース一覧.iter().any(|(パス, 行一覧)| {
            (0..行一覧.len()).filter_map(|開始| implの見出しを読む(行一覧, 開始)).any(|見出し| {
                let Some(構文) = 実装の見出しの構文::読む(&見出し.表記) else {
                    return false;
                };
                let 実装 = 読んだ実装 { パス, 行一覧, 見出し };
                let 本体 = 実装.見出し.本体の文字列(行一覧);
                構文.対象.名前() == 型.型名 && 実装.定義を指すか(&構文.対象, &定義のパス) && (本体.contains("&mut self") || (構文.対象.可変参照か && 本体.contains("fn ")))
            })
        })
    }
}
