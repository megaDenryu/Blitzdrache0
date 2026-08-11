//! 綴りを1つずつ見ながら、違反と、寄せられない綴りの台帳の陳腐化の判定に要る材料を溜める型。
//! 触れるのは自分が持つ2つの入れ物だけであり、走査もファイルの読み取りも行わない。
//!
//! 判定を型に持たせるのは、違反の生成と台帳の陳腐化の判定が同じ走査の結果を材料にするためである。
//! 別々に集めると、片方だけが古い数え方のまま残る。

use std::collections::{BTreeMap, BTreeSet};

use super::allowance;
use super::self_reference;
use super::出現箇所;
use crate::conform::violation::違反;

pub(super) struct 検査の集計 {
    違反一覧: Vec<違反>,
    綴りごとの出現場所: BTreeMap<String, BTreeSet<String>>,
}

impl 検査の集計 {
    pub(super) fn 新しく作る() -> Self {
        Self {
            違反一覧: Vec::new(),
            綴りごとの出現場所: BTreeMap::new(),
        }
    }

    pub(super) fn 綴り1つを見る(&mut self, 綴り: &str, 出現箇所一覧: &[出現箇所]) {
        let 台帳でない出現: Vec<&出現箇所> = 出現箇所一覧
            .iter()
            .filter(|出現箇所| !self_reference::台帳のファイルか(&出現箇所.パス))
            .collect();
        let ファイルごとの初出 = ファイルごとの初出を採る(&台帳でない出現);
        self.綴りごとの出現場所
            .insert(綴り.to_string(), ファイルごとの初出.keys().cloned().collect());
        if ファイルごとの初出.len() < 2 || allowance::既知の寄せられない綴りか(綴り, &台帳でない出現) {
            return;
        }
        self.違反一覧.extend(ファイルごとの初出.values().map(|出現箇所| {
            違反::行単位(
                出現箇所.パス.clone(),
                出現箇所.行番号,
                format!(
                    "ファイル名らしい綴り「{綴り}」が{}つのファイルに書かれている(正本を1箇所へ寄せる)",
                    ファイルごとの初出.len()
                ),
            )
        }));
    }

    pub(super) fn 違反一覧にする(mut self) -> Vec<違反> {
        self.違反一覧.extend(allowance::台帳の陳腐化を検査する(&self.綴りごとの出現場所));
        self.違反一覧
    }
}

/// ファイルごとに最初の出現だけを残す。違反の数え方をファイル単位に保つためであり、
/// 許可の判定は残さず全部の出現を見る。
fn ファイルごとの初出を採る<'a>(出現一覧: &[&'a 出現箇所]) -> BTreeMap<String, &'a 出現箇所> {
    let mut 初出: BTreeMap<String, &出現箇所> = BTreeMap::new();
    for 出現箇所 in 出現一覧 {
        初出.entry(出現箇所.パス.to_string_lossy().replace('\\', "/")).or_insert(出現箇所);
    }
    初出
}
