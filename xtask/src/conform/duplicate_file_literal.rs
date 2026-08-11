//! ファイル名らしい文字列リテラルが2箇所以上に書かれていないかの検査。
//! 受け取るのは無し、返すのは重複した綴りの違反一覧である。走査するファイルの範囲は`scan_scope`が持つ。
//!
//! 同じファイル名を2箇所へ書くと、片方だけを直した食い違いが実行するまで出ない。しかも綴りが合っている
//! 限り両方とも動くため、複製が増えても誰も困らないまま増え続ける。ここが機械的に止める。
//!
//! 数えるのは箇所でなくファイルである。同じファイルの中で同じ名前が複数回出るのは、台帳が1つのシェーダーへ
//! 複数の宣言を結ぶときのように、1つの持ち場の中での繰り返しであり、片方だけを直す食い違いが起こらない。

mod allowance;
mod extract;
mod scan_scope;
#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::path::PathBuf;

use super::violation::違反;

pub fn 全ファイルを検査する() -> Result<Vec<違反>, String> {
    let 出現表 = 出現箇所を集める(&scan_scope::走査するファイル一覧を集める()?)?;
    let mut 重複した綴り一覧 = Vec::new();
    let mut 違反一覧 = Vec::new();
    for (綴り, 出現箇所一覧) in 出現表 {
        if 出現箇所一覧.len() < 2 {
            continue;
        }
        重複した綴り一覧.push(綴り.clone());
        if allowance::既知の許容か(&綴り) {
            continue;
        }
        違反一覧.extend(出現箇所一覧.iter().map(|出現箇所| {
            違反::行単位(
                出現箇所.パス.clone(),
                出現箇所.行番号,
                format!(
                    "ファイル名らしい綴り「{綴り}」が{}箇所に書かれている(正本を1箇所へ寄せる)",
                    出現箇所一覧.len()
                ),
            )
        }));
    }
    違反一覧.extend(allowance::台帳の陳腐化を検査する(&重複した綴り一覧));
    Ok(違反一覧)
}

struct 出現箇所 {
    パス: PathBuf,
    行番号: usize,
}

fn 出現箇所を集める(ファイル一覧: &[PathBuf]) -> Result<BTreeMap<String, Vec<出現箇所>>, String> {
    let mut 出現表: BTreeMap<String, Vec<出現箇所>> = BTreeMap::new();
    for パス in ファイル一覧 {
        let 内容 = std::fs::read_to_string(パス).map_err(|誤り| format!("{}の読み取りに失敗した: {誤り}", パス.display()))?;
        for (綴り, 行番号) in ファイル内の初出を集める(&内容) {
            出現表.entry(綴り).or_default().push(出現箇所 {
                パス: パス.to_path_buf(),
                行番号,
            });
        }
    }
    Ok(出現表)
}

/// 1ファイルの中で綴りごとに最初の行だけを拾う。`cfg(test)`を見た行から下は読まない。
/// そこから下はファイルの途中から始まる試験であり、試験の入力として作る名前が正本を持たないためである。
fn ファイル内の初出を集める(内容: &str) -> BTreeMap<String, usize> {
    let mut 初出: BTreeMap<String, usize> = BTreeMap::new();
    for (添字, 行) in 内容.lines().enumerate() {
        if 行.contains("cfg(test)") {
            break;
        }
        for 綴り in extract::拡張子つきのリテラルを取り出す(行) {
            初出.entry(綴り).or_insert(添字 + 1);
        }
    }
    初出
}
