//! 親の型を丸ごと受け取る自由関数のうち、まだ是正できていないものの台帳と、その陳腐化の検査。
//!
//! 検査を入れた時点で178件が在り、いきなり違反にすると検証の標準列が全面的に止まる。現状を上限として
//! 登録し、増えたら落とす。台帳に載っているのに検出されなくなったときも落として台帳からの削除を強制する。
//! 減る方向の力を仕組みで用意しないと、是正が済んでも一覧が残り続けて次の違反を見逃す穴になる。
//! 様式は`xtask/src/conform/split_debt.rs`の台帳と陳腐化検査に倣う。

mod entry;
mod table;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

pub use entry::{区画の一覧, 未是正の自由関数};

use super::index::親の型を丸ごと受け取る自由関数;
use crate::conform::violation::違反;

/// 台帳の項目と検出を突き合わせる鍵。同じ関数が同じ型を2つの引数で受け取ることがあるため、
/// 違反はパスと関数名と型名の組で1件へまとめる。
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct 自由関数の位置 {
    pub パス: String,
    pub 関数名: String,
    pub 型名: String,
}

pub fn 台帳と照合する(検出一覧: &[親の型を丸ごと受け取る自由関数]) -> Vec<違反> {
    let 検出の表 = 位置ごとの行番号(検出一覧);
    let 台帳の位置一覧: BTreeSet<自由関数の位置> = table::全区画().into_iter().flat_map(|区画| 区画.位置一覧()).collect();
    let mut 違反一覧: Vec<違反> = 検出の表
        .iter()
        .filter(|(位置, _)| !台帳の位置一覧.contains(*位置))
        .map(|(位置, 行番号)| {
            違反::行単位(
                PathBuf::from(&位置.パス),
                *行番号,
                format!(
                    "自由関数「{}」が親の型「{}」を丸ごと受け取っている(工程が触るものだけを名前の付いた引数で受け取る)",
                    位置.関数名, 位置.型名
                ),
            )
        })
        .collect();
    違反一覧.extend(検出されない項目の違反(&検出の表, &台帳の位置一覧));
    違反一覧
}

/// 走査で得るパスの区切り文字は実行環境で変わるため、斜線へ揃えてから台帳と照合する。
fn 位置ごとの行番号(検出一覧: &[親の型を丸ごと受け取る自由関数]) -> BTreeMap<自由関数の位置, usize> {
    let mut 表 = BTreeMap::new();
    for 検出 in 検出一覧 {
        let 位置 = 自由関数の位置 {
            パス: 検出.パス.to_string_lossy().replace('\\', "/"),
            関数名: 検出.関数名.clone(),
            型名: 検出.型名.clone(),
        };
        表.entry(位置).or_insert(検出.行番号);
    }
    表
}

fn 検出されない項目の違反(
    検出の表: &BTreeMap<自由関数の位置, usize>, 台帳の位置一覧: &BTreeSet<自由関数の位置>
) -> Vec<違反> {
    台帳の位置一覧
        .iter()
        .filter(|位置| !検出の表.contains_key(*位置))
        .map(|位置| {
            違反::ファイル単位(
                PathBuf::from(&位置.パス),
                format!(
                    "台帳の自由関数「{}」が親の型「{}」を受け取らなくなった(台帳から削除する)",
                    位置.関数名, 位置.型名
                ),
            )
        })
        .collect()
}
