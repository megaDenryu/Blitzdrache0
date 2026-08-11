//! 綴りを1つずつ見ながら、違反と、寄せられない綴りの台帳の陳腐化の判定に要る材料を溜める型。
//! 触れるのは自分が持つ2つの列だけであり、走査もファイルの読み取りも行わない。
//!
//! 判定を型に持たせるのは、違反の生成と台帳の陳腐化の判定が同じ走査の結果を材料にするためである。
//! 別々に集めると、片方だけが古い数え方のまま残る。

use super::allowance;
use super::self_reference;
use super::出現箇所;
use crate::conform::violation::違反;

pub(super) struct 検査の集計 {
    違反一覧: Vec<違反>,
    重複した綴り一覧: Vec<String>,
}

impl 検査の集計 {
    pub(super) fn 新しく作る() -> Self {
        Self {
            違反一覧: Vec::new(),
            重複した綴り一覧: Vec::new(),
        }
    }

    pub(super) fn 綴り1つを見る(&mut self, 綴り: &str, 出現箇所一覧: &[出現箇所]) {
        let 台帳でない出現: Vec<&出現箇所> = 出現箇所一覧
            .iter()
            .filter(|出現箇所| !self_reference::台帳のファイルか(&出現箇所.パス))
            .collect();
        if 台帳でない出現.len() < 2 {
            return;
        }
        self.重複した綴り一覧.push(綴り.to_string());
        if allowance::既知の寄せられない綴りか(綴り, &台帳でない出現) {
            return;
        }
        self.違反一覧.extend(台帳でない出現.iter().map(|出現箇所| {
            違反::行単位(
                出現箇所.パス.clone(),
                出現箇所.行番号,
                format!(
                    "ファイル名らしい綴り「{綴り}」が{}つのファイルに書かれている(正本を1箇所へ寄せる)",
                    台帳でない出現.len()
                ),
            )
        }));
    }

    pub(super) fn 違反一覧にする(mut self) -> Vec<違反> {
        self.違反一覧.extend(allowance::台帳の陳腐化を検査する(&self.重複した綴り一覧));
        self.違反一覧
    }
}
