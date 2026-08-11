//! 通常文字列の逃がし記号を、実行時の文字へ復号する工程。受け取るのは原文のままの中身、返すのは実行時の中身である。
//!
//! 生文字列は逃がしを持たないため復号を通さない。両方を実行時の中身へ揃えないと、`"a\x2eb"`と`"a.b"`が
//! 同じ名前を指すのに別の綴りとして数えられ、片方が重複の検出から消える。
//!
//! 綴りとして成り立たない逃がしは原文のまま残す。rustcが先に弾くため、ここが誤りを報告する立場ではない。

use std::iter::Peekable;
use std::str::Chars;

pub(super) fn 逃がしを復号する(原文: &str) -> String {
    let mut 出力 = String::new();
    let mut 残り = 原文.chars().peekable();
    while let Some(文字) = 残り.next() {
        if 文字 != '\\' {
            出力.push(文字);
            continue;
        }
        let Some(印) = 残り.next() else {
            出力.push('\\');
            break;
        };
        印を復号する(印, &mut 残り, &mut 出力);
    }
    出力
}

fn 印を復号する(印: char, 残り: &mut Peekable<Chars<'_>>, 出力: &mut String) {
    match 印 {
        'n' => 出力.push('\n'),
        'r' => 出力.push('\r'),
        't' => 出力.push('\t'),
        '0' => 出力.push('\0'),
        '\\' | '"' | '\'' => 出力.push(印),
        'x' => 十六進の2桁を復号する(残り, 出力),
        'u' => 波括弧の符号位置を復号する(残り, 出力),
        // 行末の逃がしは改行と、続く字下げを消す。
        '\n' => while 残り.next_if(|文字| 文字.is_whitespace()).is_some() {},
        _ => {
            出力.push('\\');
            出力.push(印);
        }
    }
}

fn 十六進の2桁を復号する(残り: &mut Peekable<Chars<'_>>, 出力: &mut String) {
    let 桁: String = (0..2).filter_map(|_| 残り.next_if(char::is_ascii_hexdigit)).collect();
    文字へ写して押す(&桁, 出力, &format!("\\x{桁}"));
}

/// `\u{2e}`と`\u{2_e}`。下線は桁の区切りとして書けるため、読み飛ばしてから数として写す。
/// 閉じの波括弧が無い形は綴りとして成り立たないため、読んだところまでを原文のまま残す。
fn 波括弧の符号位置を復号する(残り: &mut Peekable<Chars<'_>>, 出力: &mut String) {
    if 残り.next_if(|文字| *文字 == '{').is_none() {
        出力.push_str("\\u");
        return;
    }
    let 原文の桁: String = std::iter::from_fn(|| 残り.next_if(|文字| 文字.is_ascii_hexdigit() || *文字 == '_')).collect();
    if 残り.next_if(|文字| *文字 == '}').is_none() {
        出力.push_str(&format!("\\u{{{原文の桁}"));
        return;
    }
    let 桁: String = 原文の桁.chars().filter(|文字| *文字 != '_').collect();
    文字へ写して押す(&桁, 出力, &format!("\\u{{{原文の桁}}}"));
}

/// 十六進の桁を文字へ写す。写せない桁は原文のまま残す。
fn 文字へ写して押す(桁: &str, 出力: &mut String, 原文: &str) {
    let 文字 = u32::from_str_radix(桁, 16).ok().and_then(char::from_u32);
    match 文字 {
        Some(文字) => 出力.push(文字),
        None => 出力.push_str(原文),
    }
}
