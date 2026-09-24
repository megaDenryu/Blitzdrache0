//! 字句の木の並びの中の行の頭の `impl` と `type` の後ろの字句から、見出しの中身(`header_content.rs`)を取り出す純粋な関数と、取り出した行の頭の見出し1件の値。受け取るのは proc-macro2 の字句の並びと予約語の添字、返すのは見出しの中身か行の頭の見出しである。
//! 字句の木の側で取り出すのは、読み口が文字列で読んだ値と答え合わせをするためである。字句の木を基準にするのは、識別子の文字・コメント・括弧の対応を rustc と同じ規則で読むためであり、構文の木は作らない。
//! **字句の木の側は、読み口と読み方の規則を共有しない。** 同じ規則を両側で使うと、その規則が誤ったとき両側が同じ誤りに揃い、突き合わせが食い違いを出さないためである。
//! そのため型引数の名前は、`Ident`・`Punct`・`Group` の並びとして読む。外側の属性(`#` と角括弧の群)を読み飛ばした後が寿命(`'`)なら名前を持たず、`const` なら次の識別子、それ以外なら最初の識別子を名前にする。
//! 見出しの終わりは、山括弧の外の本体の波括弧の群か `where` であり、括弧の中は群として1つの字句になるため、型引数の定数式の中の `where` で止まらない。山括弧の外の探索は `angle_bracket_scan.rs`、型の表記の中身と名前は `type_notation_name.rs` が持つ。

use proc_macro2::{Delimiter, TokenTree};

use super::angle_bracket_scan::{最上位で止める, 最上位のカンマで分ける, 最上位の位置, 記号か, 語か, 高階の寿命の束縛か};
use super::header_content::見出しの中身;
use super::item_keyword_position::項目の予約語;
use super::normalized_token_sequence::正規化した字句の並び;
use super::type_notation_name::{パスの最後の名前, 対象の型の中身};

/// 行の頭の `impl` と `type` の後ろの字句から取り出した見出しの中身1件。行番号は1始まりである。
pub struct 行の頭の見出し {
    pub 行番号: usize,
    pub 予約語: 項目の予約語,
    pub 中身: 見出しの中身,
}

/// 並びの添字の行の頭の予約語の後ろの字句から、行番号の行の見出しを取り出す。見出しの中身を持つのは `impl` と `type` だけであり、`use` と `macro_rules!` なら無い。
pub fn 行の頭の見出しを取り出す(字句一覧: &[TokenTree], 添字: usize, 予約語: 項目の予約語, 行番号: usize) -> Option<行の頭の見出し> {
    let 中身 = match 予約語 {
        項目の予約語::実装 => Some(実装の見出しを取り出す(字句一覧, 添字)),
        項目の予約語::型の別名 => 型の別名の見出しを取り出す(字句一覧, 添字),
        項目の予約語::取り込み | 項目の予約語::マクロの定義 => None,
    }?;
    Some(行の頭の見出し { 行番号, 予約語, 中身 })
}

/// 並びの添字の `impl` の後ろの字句から、実装の見出しの中身を取り出す。見出しは本体を開く波括弧の群か `where` までである。
pub fn 実装の見出しを取り出す(字句一覧: &[TokenTree], 添字: usize) -> 見出しの中身 {
    let (型引数の名前一覧, 後ろ) = 先頭の型引数を分ける(字句一覧.get(添字 + 1..).unwrap_or_default());
    let 宣言 = 最上位で止める(後ろ, |字句| matches!(字句, TokenTree::Group(群) if 群.delimiter() == Delimiter::Brace) || 語か(字句, "where"));
    let forの位置 = 最上位の位置(宣言, |位置| 語か(&宣言[位置], "for") && !高階の寿命の束縛か(宣言.get(位置 + 1..).unwrap_or_default()));
    let (トレイトの実装か, 対象) = forの位置.map_or((false, 宣言), |位置| (true, 宣言.get(位置 + 1..).unwrap_or_default()));
    let (中身, 可変参照を対象にするか) = 対象の型の中身(対象);
    見出しの中身::実装 {
        トレイトの実装か,
        対象の型の名前: パスの最後の名前(&中身),
        対象の型の表記: Some(正規化した字句の並び::字句から作る(&中身)),
        可変参照を対象にするか,
        型引数の名前一覧,
    }
}

/// 並びの添字の `type` の後ろの字句から、型の別名の見出しの中身を取り出す。宣言は最上位の `;` までである。`type` の後ろが識別子でなければ無い。
pub fn 型の別名の見出しを取り出す(字句一覧: &[TokenTree], 添字: usize) -> Option<見出しの中身> {
    let Some(TokenTree::Ident(別名)) = 字句一覧.get(添字 + 1) else {
        return None;
    };
    let (型引数の名前一覧, 後ろ) = 先頭の型引数を分ける(字句一覧.get(添字 + 2..).unwrap_or_default());
    let 宣言 = 最上位で止める(後ろ, |字句| 記号か(字句, ';'));
    Some(match 最上位の位置(宣言, |位置| 記号か(&宣言[位置], '=')) {
        None => 見出しの中身::右辺の無い型の別名,
        Some(位置) => {
            let 右辺 = 宣言.get(位置 + 1..).unwrap_or_default();
            見出しの中身::右辺のある型の別名 {
                別名: 別名.to_string(),
                型引数の名前一覧,
                右辺の最後の名前: パスの最後の名前(右辺),
                右辺: Some(正規化した字句の並び::字句から作る(右辺)),
            }
        }
    })
}

// 先頭が `<` なら、閉じる `>` までの型引数の名前の一覧と、その後ろの字句。先頭が `<` でなければ、空の一覧と字句のすべて。
fn 先頭の型引数を分ける(字句一覧: &[TokenTree]) -> (Vec<String>, &[TokenTree]) {
    if !字句一覧.first().is_some_and(|字句| 記号か(字句, '<')) {
        return (Vec::new(), 字句一覧);
    }
    let 閉じる位置 = 最上位の位置(字句一覧, |位置| 位置 > 0 && 記号か(&字句一覧[位置], '>')).unwrap_or(字句一覧.len());
    let 中 = 字句一覧.get(1..閉じる位置).unwrap_or_default();
    (最上位のカンマで分ける(中).into_iter().filter_map(型引数の名前).collect(), 字句一覧.get(閉じる位置 + 1..).unwrap_or_default())
}

// 型引数1つの字句の名前。外側の属性(`#` と角括弧の群)を読み飛ばし、寿命の引数は名前を持たず、定数の引数は `const` の次の識別子、それ以外は最初の識別子である。
fn 型引数の名前(引数: &[TokenTree]) -> Option<String> {
    let mut 残り = 引数;
    while let [TokenTree::Punct(記号), TokenTree::Group(群), 後ろ @ ..] = 残り {
        if 記号.as_char() != '#' || 群.delimiter() != Delimiter::Bracket {
            break;
        }
        残り = 後ろ;
    }
    match 残り {
        [TokenTree::Ident(語), TokenTree::Ident(名前), ..] if 語 == "const" => Some(名前.to_string()),
        [TokenTree::Ident(名前), ..] => Some(名前.to_string()),
        [..] => None,
    }
}
