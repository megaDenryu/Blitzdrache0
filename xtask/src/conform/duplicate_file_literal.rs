//! ファイル名らしい文字列リテラルが2つ以上のファイルに書かれていないかの検査。
//! 受け取るのは無し、返すのは重複した綴りの違反一覧である。走査するファイルの範囲は`scan_scope`が持つ。
//!
//! 同じファイル名を2箇所へ書くと、片方だけを直した食い違いが実行するまで出ない。しかも綴りが合っている
//! 限り両方とも動くため、複製が増えても誰も困らないまま増え続ける。ここが機械的に止める。
//!
//! 数えるのは箇所でなくファイルである。同じファイルの中で同じ名前が複数回出るのは、1つの持ち場の中での
//! 繰り返しであり、片方だけを直す食い違いが起こらない。
//! 自分で検査の対象を名指しする台帳は数から外す。どのファイルを外したかは`self_reference`が1件ずつ持つ。

mod allowance;
mod extract;
mod scan_scope;
mod self_reference;
mod tally;
mod test_item_skip;
#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::path::PathBuf;

use super::source_lexing;
use super::violation::違反;
use tally::検査の集計;

pub fn 全ファイルを検査する() -> Result<Vec<違反>, String> {
    let 出現表 = 出現箇所を集める(&scan_scope::走査するファイル一覧を集める()?)?;
    let mut 集計 = 検査の集計::新しく作る();
    for (綴り, 出現箇所一覧) in 出現表 {
        集計.綴り1つを見る(&綴り, &出現箇所一覧);
    }
    let mut 違反一覧 = 集計.違反一覧にする();
    違反一覧.extend(self_reference::一覧の陳腐化を検査する()?);
    Ok(違反一覧)
}

struct 出現箇所 {
    パス: PathBuf,
    行番号: usize,
    /// その行が`include_bytes!`の呼び出しか。ビルド時に焼いた成果物を取り込む形かどうかの判定に使う。
    取り込みの引数か: bool,
}

fn 出現箇所を集める(ファイル一覧: &[PathBuf]) -> Result<BTreeMap<String, Vec<出現箇所>>, String> {
    let mut 出現表: BTreeMap<String, Vec<出現箇所>> = BTreeMap::new();
    for パス in ファイル一覧 {
        let 内容 = std::fs::read_to_string(パス).map_err(|誤り| format!("{}の読み取りに失敗した: {誤り}", パス.display()))?;
        for (綴り, 要点) in ファイル内の初出を集める(&内容) {
            出現表.entry(綴り).or_default().push(出現箇所 {
                パス: パス.to_path_buf(),
                行番号: 要点.行番号,
                取り込みの引数か: 要点.取り込みの引数か,
            });
        }
    }
    Ok(出現表)
}

/// 1ファイルの中で綴りごとに最初の行だけを拾う。`#[cfg(test)]`の付いた項目の中は数えない。
/// 試験の入力として作る名前が正本を持たないためである。
fn ファイル内の初出を集める(内容: &str) -> BTreeMap<String, 行の要点> {
    let 試験の範囲一覧 = test_item_skip::試験の項目の行範囲一覧(内容);
    let コード行一覧 = source_lexing::コードだけの行一覧(内容);
    let mut 初出: BTreeMap<String, 行の要点> = BTreeMap::new();
    for 断片 in source_lexing::文字列リテラル一覧(内容) {
        if test_item_skip::範囲の中の行か(&試験の範囲一覧, 断片.開始行) || !extract::拡張子を含むか(&断片.中身) {
            continue;
        }
        初出.entry(extract::ファイル名の部分(&断片.中身)).or_insert(行の要点 {
            行番号: 断片.開始行,
            取り込みの引数か: 取り込みの行か(&コード行一覧, 断片.開始行),
        });
    }
    初出
}

/// 1つの出現について、行番号とその行の性質を持つ。ファイルの中で綴りごとに1つだけ作る。
struct 行の要点 {
    行番号: usize,
    取り込みの引数か: bool,
}

/// その行がビルド時の成果物を取り込む呼び出しか。判定にコードだけの行を使うのは、コメントや文字列の中の
/// 語を呼び出しと読み違えないためである。
fn 取り込みの行か(コード行一覧: &[String], 行番号: usize) -> bool {
    コード行一覧.get(行番号 - 1).is_some_and(|行| 行.contains(取り込みの呼び出し))
}

const 取り込みの呼び出し: &str = "include_bytes!";
