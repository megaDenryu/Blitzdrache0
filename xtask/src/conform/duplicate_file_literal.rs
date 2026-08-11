//! ファイル名らしい文字列リテラルが2つ以上のファイルに書かれていないかの検査。
//! 受け取るのは無し、返すのは重複した綴りの違反一覧である。走査するファイルの範囲は`scan_scope`が持つ。
//!
//! 同じファイル名を2箇所へ書くと、片方だけを直した食い違いが実行するまで出ない。しかも綴りが合っている
//! 限り両方とも動くため、複製が増えても誰も困らないまま増え続ける。ここが機械的に止める。
//!
//! 違反として数えるのは箇所でなくファイルである。同じファイルの中で同じ名前が複数回出るのは、1つの持ち場の
//! 中での繰り返しであり、片方だけを直す食い違いが起こらない。ただし出現そのものは1つ残らず集める。
//! 許可の規則が「ほかの出現が全部ビルド成果物の取り込みか」を見るため、同じファイルの2つ目以降を
//! 落とすと、取り込みの後ろに書いた別の用途の出現が判定から消える。
//! 自分で検査の対象を名指しする台帳は数から外す。どのファイルを外したかは`self_reference`が1件ずつ持つ。

mod allowance;
mod builtin_include_bytes;
#[cfg(test)]
mod builtin_include_bytes_tests;
mod extract;
mod include_bytes_argument;
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
    let ファイル一覧 = scan_scope::走査するファイル一覧を集める()?;
    let mut 集計 = 検査の集計::新しく作る();
    for (綴り, 出現箇所一覧) in 出現箇所を集める(&ファイル一覧)? {
        集計.綴り1つを見る(&綴り, &出現箇所一覧);
    }
    let mut 違反一覧 = 集計.違反一覧にする();
    違反一覧.extend(builtin_include_bytes::全ファイルを検査する(&ファイル一覧)?);
    違反一覧.extend(self_reference::一覧の陳腐化を検査する()?);
    Ok(違反一覧)
}

pub(super) struct 出現箇所 {
    pub(super) パス: PathBuf,
    pub(super) 行番号: usize,
    /// その綴りが`include_bytes!`の引数の中に在るか。ビルド時に焼いた成果物の取り込みかの判定に使う。
    pub(super) 取り込みの引数か: bool,
}

fn 出現箇所を集める(ファイル一覧: &[PathBuf]) -> Result<BTreeMap<String, Vec<出現箇所>>, String> {
    let mut 出現表: BTreeMap<String, Vec<出現箇所>> = BTreeMap::new();
    for パス in ファイル一覧 {
        let 内容 = std::fs::read_to_string(パス).map_err(|誤り| format!("{}の読み取りに失敗した: {誤り}", パス.display()))?;
        for (綴り, 行番号, 取り込みの引数か) in ファイル内の出現を集める(&内容) {
            出現表.entry(綴り).or_default().push(出現箇所 {
                パス: パス.to_path_buf(),
                行番号,
                取り込みの引数か,
            });
        }
    }
    Ok(出現表)
}

/// 1ファイルの中のファイル名らしい綴りを、1つ残らず集める。`#[cfg(test)]`の付いた項目の中は数えない。
/// 試験の入力として作る名前が正本を持たないためである。
fn ファイル内の出現を集める(内容: &str) -> Vec<(String, usize, bool)> {
    let 試験の範囲一覧 = test_item_skip::試験の項目の行範囲一覧(内容);
    let 断片一覧 = source_lexing::字句へ分ける(内容);
    let 取り込みの区間一覧 = include_bytes_argument::取り込みの引数の区間一覧(&断片一覧);
    断片一覧
        .iter()
        .filter(|断片| 断片.区分 == source_lexing::字句の区分::文字列リテラル)
        .filter(|断片| !test_item_skip::範囲の中の行か(&試験の範囲一覧, 断片.開始行) && extract::拡張子を含むか(&断片.中身))
        .map(|断片| {
            (
                extract::ファイル名の部分(&断片.中身),
                断片.開始行,
                include_bytes_argument::区間の中か(&取り込みの区間一覧, 断片.開始位置),
            )
        })
        .collect()
}
