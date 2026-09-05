//! `use`宣言1件を、持ち込まれた名前ごとの経路へ展開する。`super::{a::A, B}`は`super::a::A`と`super::B`になる。
//!
//! 読み取れない形は経路を作らずに落とす。`*`での持ち込みは名前が書かれておらず、`as`での改名は綴られた経路と
//! ファイルの中の名前が食い違うためであり、どちらも「どの定義か」を経路から言えない。

const 段の区切り: &str = "::";
const 改名の綴り: &str = " as ";
const 全部を持ち込む綴り: &str = "*";

pub fn 経路へ展開する(前置き: &str, 項: &str) -> Vec<String> {
    let 項 = 項.trim();
    let Some(開き) = 項.find('{') else {
        return 単独の項を経路にする(前置き, 項);
    };
    let (Some(閉じ), Some(頭)) = (項.rfind('}'), 項.get(..開き)) else {
        return Vec::new();
    };
    let Some(中身) = 項.get(開き + 1..閉じ) else {
        return Vec::new();
    };
    let 新しい前置き = 経路を繋ぐ(前置き, 頭.trim().trim_end_matches(段の区切り));
    読点で分ける(中身).iter().flat_map(|子| 経路へ展開する(&新しい前置き, 子)).collect()
}

fn 単独の項を経路にする(前置き: &str, 項: &str) -> Vec<String> {
    if 項.is_empty() || 項.contains(改名の綴り) || 項.ends_with(全部を持ち込む綴り) {
        return Vec::new();
    }
    vec![経路を繋ぐ(前置き, 項)]
}

fn 経路を繋ぐ(前置き: &str, 続き: &str) -> String {
    match (前置き.is_empty(), 続き.is_empty()) {
        (true, _) => 続き.to_string(),
        (false, true) => 前置き.to_string(),
        (false, false) => format!("{前置き}{段の区切り}{続き}"),
    }
}

/// 波括弧の入れ子の中の読点で切らないよう、深さ0の読点だけで分ける。
fn 読点で分ける(中身: &str) -> Vec<String> {
    let mut 一覧 = Vec::new();
    let mut 途中 = String::new();
    let mut 深さ: usize = 0;
    for 文字 in 中身.chars() {
        match 文字 {
            '{' => 深さ += 1,
            '}' => 深さ = 深さ.saturating_sub(1),
            ',' if 深さ == 0 => {
                一覧.push(std::mem::take(&mut 途中));
                continue;
            }
            _ => {}
        }
        途中.push(文字);
    }
    一覧.push(途中);
    一覧
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 波括弧の中の名前をそれぞれの経路にする() {
        assert_eq!(
            経路へ展開する("", "super::{ 建物外形カタログ, 建物外形定義 }"),
            vec!["super::建物外形カタログ".to_string(), "super::建物外形定義".to_string()]
        );
    }

    #[test]
    fn 入れ子の波括弧も段を繋いで展開する() {
        assert_eq!(
            経路へ展開する("", "crate::{cli::{設定, 指定}, 起動}"),
            vec!["crate::cli::設定".to_string(), "crate::cli::指定".to_string(), "crate::起動".to_string()]
        );
    }

    #[test]
    fn 改名と全部の持ち込みは経路にしない() {
        assert!(経路へ展開する("", "crate::far::設定 as 遠い設定").is_empty());
        assert!(経路へ展開する("", "crate::far::*").is_empty());
        assert_eq!(経路へ展開する("", "crate::{far::*, near::設定}"), vec!["crate::near::設定".to_string()]);
    }
}
