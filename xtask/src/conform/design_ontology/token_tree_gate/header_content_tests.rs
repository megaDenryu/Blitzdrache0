//! 見出しの中身の突き合わせの試験。読み口と字句の木が同じ見出しを同じ中身に読むことと、中身が食い違えば違反にすることを固定する。検収が見つけた読み違いの反例は `misreading_counterexample_tests.rs` が持つ。

use std::path::Path;
use std::str::FromStr;

use proc_macro2::{TokenStream, TokenTree};

use super::super::declaration_reading_outcome::宣言を読んだ結末;
use super::super::impl_syntax::実装の見出しの構文;
use super::super::type_alias_scan::型の別名の宣言を読む;
use super::header_content::見出しの中身;
use super::header_content_reconciliation::見出しの中身が食い違った違反一覧;
use super::header_tokens::{型の別名の見出しを取り出す, 実装の見出しを取り出す};
use super::item_keyword_position::項目の予約語;
use super::reader_answer::{読み口の答え, 読み口の結末};
use super::token_tree_scan::行の頭の見出し;

// 本文を字句の木の並びへ変え、最初に現れる予約語 `予約語` の添字と組にする。
#[allow(clippy::expect_used)]
fn 字句一覧と予約語の添字(本文: &str, 予約語: &str) -> (Vec<TokenTree>, usize) {
    let 字句一覧: Vec<TokenTree> = TokenStream::from_str(本文).expect("字句の木へ変える").into_iter().collect();
    let 添字 = 字句一覧.iter().position(|字句| matches!(字句, TokenTree::Ident(語) if 語 == 予約語)).expect("予約語がある");
    (字句一覧, 添字)
}

#[test]
#[allow(clippy::expect_used)]
fn 読み口と字句の木は実装の見出しを同じ中身に読む() {
    for 見出し in [
        "impl 規則 {}",
        "impl<'a, T: Into<Vec<u8>>, const N: usize> crate::a::規則<'a, T, N> where T: Clone {}",
        "unsafe impl<T·x: M不変データ> 変更 for T·x {}",
        "impl 型·for {}",
        "impl M不変データ for::jibun::規則 {}",
        "impl 変更 for &'a mut (規則) {}",
        "impl 変更 for std::pin::Pin<&mut 規則> {}",
        "impl 変更 for (規則, u8) {}",
        "impl 変更 for [規則] {}",
        "impl dyn for<'a> Fn(&'a u8) {}",
        "impl<空間種: $crate::空間> std::ops::Add for $型<空間種> {}",
        "impl<L> Service<axum::serve::IncomingStream<'_, L>> for 経路正規化アプリ {}",
        "impl !Send for 規則 {}",
        "impl 変更 for fn(u8) -> 規則 {}",
    ] {
        let 読み口 = 見出しの中身::実装の見出しの構文から作る(&実装の見出しの構文::読む(見出し).expect("見出しを読む"));
        let (字句一覧, 添字) = 字句一覧と予約語の添字(見出し, "impl");
        assert_eq!(実装の見出しを取り出す(&字句一覧, 添字), 読み口, "{見出し}");
    }
}

#[test]
fn 読み口と字句の木は型の別名を同じ中身に読む() {
    for 宣言 in [
        "type 別名 = 規則;",
        "type 包み<'a, T> = Vec<&'a T>;",
        "type 配列 = [規則; 2];",
        "type 関数 = fn(&u8) -> u8;",
        "type 同じ·x<T = 規則> = T;",
        "type 射影 = <甲 as 乙>::丙;",
        "type 状態: M状態 + PartialEq<u8>;",
    ] {
        let 読み口 = match 型の別名の宣言を読む(&[宣言.to_string()], 0) {
            宣言を読んだ結末::読めた(読んだ) => Some(見出しの中身::型の別名の宣言から作る(&読んだ)),
            宣言を読んだ結末::辺を作らない宣言を読んだ => Some(見出しの中身::右辺の無い型の別名),
            宣言を読んだ結末::その宣言でない | 宣言を読んだ結末::読めない(_) => None,
        };
        let (字句一覧, 添字) = 字句一覧と予約語の添字(宣言, "type");
        assert_eq!(型の別名の見出しを取り出す(&字句一覧, 添字), 読み口, "{宣言}");
    }
}

#[test]
fn 中身が食い違った見出しを違反にする() {
    let 字句の木 = 見出しの中身::実装 {
        トレイトの実装か: false,
        対象の型の名前: Some("型·for".to_string()),
        型引数の名前一覧: Vec::new(),
    };
    let 読み違い = 見出しの中身::実装 {
        トレイトの実装か: true,
        対象の型の名前: None,
        型引数の名前一覧: Vec::new(),
    };
    let 見出し一覧 = [行の頭の見出し {
        行番号: 2,
        予約語: 項目の予約語::実装,
        中身: 字句の木.clone(),
    }];
    for (中身, 件数) in [(読み違い, 1), (字句の木, 0)] {
        let 答え一覧 = [読み口の答え {
            行番号: 2,
            予約語: 項目の予約語::実装,
            結末: 読み口の結末::見出しを読めた(中身),
        }];
        assert_eq!(見出しの中身が食い違った違反一覧(Path::new("crates/a/src/x.rs"), &見出し一覧, &答え一覧).len(), 件数);
    }
}
