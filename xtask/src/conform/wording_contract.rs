//! アプリが出す行の綴りと、それを読む検収の綴りの突き合わせ。
//! 受け取るのは無し(対象の綴りは表が持つ)、返すのは綴りが消えたファイルの違反一覧である。
//!
//! アプリの標準出力は検収との機械の契約であり、綴りが1文字違うと検収は行を見つけられない。
//! 見つからないときの失敗はその場では「値が無い」としか出ず、どちらの側を直せばよいか読み手には分からない。
//! 台帳が契約の綴りを1つ持ち、その綴りが現れるべきファイルを列挙することで、片側だけの書き換えを検査が拒む。
//!
//! 出現と数えるのは文字列リテラルの中だけである。コメントでの言及を出現と数えると、出す側の行を消しても
//! 「綴りに触れたコメント」1つで検査が通ってしまい、この検査が反証力を失う。
//! 判定は`source_lexing`の字句の走査が持つ。

mod table;
#[cfg(test)]
mod tests;
mod unregistered_key;

use std::path::{Path, PathBuf};

use super::error::規約検査の破れ;
use super::source_lexing;
use super::violation::違反;

pub fn 全綴りを検査する() -> Result<Vec<違反>, 規約検査の破れ> {
    let mut 違反一覧 = 登録した綴りが両側に在るかを検査する()?;
    違反一覧.extend(unregistered_key::登録の無い鍵を探す(
        &登録済みの綴り一覧(),
        &台帳に載る全ファイル(),
    )?);
    Ok(違反一覧)
}

/// 台帳が登録した綴りが、並べたファイルのすべてに文字列リテラルとして在るか。
fn 登録した綴りが両側に在るかを検査する() -> Result<Vec<違反>, 規約検査の破れ> {
    let mut 違反一覧 = Vec::new();
    for 契約 in table::領域一覧.iter().copied().flatten() {
        for パス in 契約.現れるファイル一覧 {
            let 内容 = std::fs::read_to_string(Path::new(パス))
                .map_err(|誤り| 規約検査の破れ::ファイルを読めなかった(Path::new(パス), 誤り))?;
            if !文字列リテラルの中に現れるか(&内容, 契約.綴り) {
                違反一覧.push(違反::ファイル単位(
                    PathBuf::from(*パス),
                    format!(
                        "契約した綴り「{}」が文字列リテラルとして無い(この綴りを変えるなら{}の全部とconformの台帳を同時に直す)",
                        契約.綴り,
                        契約.現れるファイル一覧.join("・")
                    ),
                ));
            }
        }
    }
    Ok(違反一覧)
}

/// 台帳に載る綴りの全部。鍵が登録済みかの引き当てに使う。
fn 登録済みの綴り一覧() -> Vec<&'static str> {
    table::領域一覧.iter().copied().flatten().map(|契約| 契約.綴り).collect()
}

/// 台帳に載るファイルの全部。同じファイルが複数の契約に並ぶため、重複を畳んでから返す。
fn 台帳に載る全ファイル() -> Vec<&'static str> {
    let mut 一覧: Vec<&'static str> = table::領域一覧
        .iter()
        .copied()
        .flatten()
        .flat_map(|契約| 契約.現れるファイル一覧.iter().copied())
        .collect();
    一覧.sort_unstable();
    一覧.dedup();
    一覧
}

fn 文字列リテラルの中に現れるか(内容: &str, 綴り: &str) -> bool {
    source_lexing::文字列リテラル一覧(内容).iter().any(|断片| 断片.中身.contains(綴り))
}
