//! 1行から、ファイル名らしい文字列リテラルを取り出す工程。受け取るのは行1本、返すのはその行に在った
//! 拡張子つきのリテラルの中身の一覧である。
//!
//! 拡張子とみなすのは、点に続く英字始まりの英数字が2文字以上10文字以下で、その後が英数字でない並びである。
//! 英字始まりに限るのは、`0.05`のような数の綴りを拡張子と読み違えないためである。

/// その行に在る文字列リテラルのうち、拡張子つきのものの中身。二重引用符は含めない。
pub(super) fn 拡張子つきのリテラルを取り出す(行: &str) -> Vec<String> {
    リテラルを取り出す(行).into_iter().filter(|中身| 拡張子を含むか(中身)).collect()
}

/// 二重引用符で挟まれた区間を順に取り出す。逃がし記号の入ったリテラルは扱わず、切れ目がずれた区間は
/// 次の判定で拡張子を持たない綴りとして落ちる。
fn リテラルを取り出す(行: &str) -> Vec<String> {
    let mut 一覧 = Vec::new();
    let mut 中身 = String::new();
    let mut 開いている = false;
    for 文字 in 行.chars() {
        if 文字 == '"' {
            if 開いている {
                一覧.push(std::mem::take(&mut 中身));
            }
            開いている = !開いている;
            continue;
        }
        if 開いている {
            中身.push(文字);
        }
    }
    一覧
}

fn 拡張子を含むか(中身: &str) -> bool {
    中身.match_indices('.').any(|(位置, _)| {
        let 続き: String = 中身[位置 + 1..].chars().take_while(|文字| 文字.is_ascii_alphanumeric()).collect();
        let 長さが拡張子の範囲 = (2..=10).contains(&続き.chars().count());
        let 英字始まり = 続き.chars().next().is_some_and(|文字| 文字.is_ascii_alphabetic());
        let 続きの後ろが英数字でない = 中身[位置 + 1 + 続き.len()..]
            .chars()
            .next()
            .is_none_or(|文字| !文字.is_ascii_alphanumeric());
        長さが拡張子の範囲 && 英字始まり && 続きの後ろが英数字でない
    })
}
