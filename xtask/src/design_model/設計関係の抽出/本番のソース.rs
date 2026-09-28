//! クレートの入口から本番のモジュール宣言を辿り、抽出規則が共有するソース一覧を組む。
//! 試験用の原文では入口を明示して与える。本番の走査はlib.rsとmain.rsだけを入口にする。
//! Graphiteの生成物だけを取り込む波括弧付きのモジュールは辿らず、明示して除外した行として数える(2026-09-26。Issue #187)。
//! 生成物と認めるかは `conform::graphiteのコード` が見出しと生成元の宣言の結び付きで決め、この工程はその判定を借りる。

use super::ontology_scope::適用範囲のクレートのパスか;
use super::本番の行::本番の行を選ぶ;
use super::{保証範囲の外の構文, 抽出できなかった理由, 抽出できなかった行};
use crate::conform::design_ontology::module_declaration::{モジュール宣言, 置き場の指定};
use crate::conform::design_ontology::module_declaration_extract::モジュール宣言の抽出;
use crate::conform::design_ontology::module_path::ソースのファイルの末尾;
use crate::conform::graphiteのコード::Graphiteの生成物の一覧;
use crate::conform::走査した原文の一覧::{斜線で揃えたパス, 走査した原文の一覧};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub struct 本番のソース {
    pub 行一覧: Vec<(PathBuf, Vec<String>)>,
    pub 欠落一覧: Vec<抽出できなかった行>,
}

/// 適用範囲のクレートの原文と、その中のGraphiteの生成物の一覧。モジュールの宣言1つがどこへ辿れるかを答える。
struct 適用範囲の原文 {
    原文一覧: 走査した原文の一覧,
    生成物: Graphiteの生成物の一覧,
}

/// モジュールの宣言1つを辿った先。
enum 宣言の行き先 {
    本番の項目でない,
    子を辿る(PathBuf),
    明示して除外する(抽出できなかった行),
    辿れない(抽出できなかった行),
}

impl 本番のソース {
    pub fn 入口から集める(原文一覧: Vec<(PathBuf, String)>, 入口一覧: Vec<PathBuf>) -> Self {
        let 範囲 = 適用範囲の原文::生成する(原文一覧);
        let mut 待ち = 入口一覧;
        let mut 済み = HashSet::new();
        let mut 本番 = Self {
            行一覧: Vec::new(), 欠落一覧: Vec::new()
        };
        while let Some(パス) = 待ち.pop() {
            if !済み.insert(斜線で揃えたパス::生成する(&パス)) {
                continue;
            }
            let Some(原文) = 範囲.原文一覧.パスで探す(&パス) else {
                continue;
            };
            let (mut 行一覧, 欠落) = 本番の行を選ぶ(&パス, 原文);
            let mut 子一覧 = Vec::new();
            let mut モジュールの欠落 = Vec::new();
            for 宣言 in モジュール宣言の抽出::生成する(パス.clone(), 原文).宣言一覧() {
                match 範囲.宣言の行き先(&パス, 原文, &行一覧, &宣言) {
                    宣言の行き先::本番の項目でない => {}
                    宣言の行き先::子を辿る(子) => 子一覧.push(子),
                    宣言の行き先::明示して除外する(行) => 本番.欠落一覧.push(行),
                    宣言の行き先::辿れない(行) => モジュールの欠落.push(行),
                }
            }
            if モジュールの欠落.is_empty() {
                待ち.extend(子一覧);
            } else {
                行一覧.iter_mut().for_each(String::clear);
            }
            本番.欠落一覧.extend(モジュールの欠落);
            本番.欠落一覧.extend(欠落);
            本番.行一覧.push((パス, 行一覧));
        }
        本番.行一覧.sort_by(|左, 右| 左.0.cmp(&右.0));
        本番
    }
}

impl 適用範囲の原文 {
    fn 生成する(原文一覧: Vec<(PathBuf, String)>) -> Self {
        let 原文一覧 = 走査した原文の一覧::生成する(原文一覧.into_iter().filter(|(パス, _)| 適用範囲のクレートのパスか(パス)).collect());
        let 生成物 = Graphiteの生成物の一覧::原文一覧から見分ける(&原文一覧);
        Self { 原文一覧, 生成物 }
    }

    /// 試験の項目の宣言は本番の項目でない。生成物の取り込みは明示して除外し、本体が別ファイルに在ればそれを辿り、どちらでもなければ辿れない。
    fn 宣言の行き先(&self, パス: &Path, 原文: &str, 本番の行一覧: &[String], 宣言: &モジュール宣言) -> 宣言の行き先 {
        if 本番の行一覧.get(宣言.行番号 - 1).is_none_or(|行| 行.trim().is_empty()) {
            return 宣言の行き先::本番の項目でない;
        }
        if self.生成物.波括弧の中で取り込む生成物(パス, 原文, 宣言.行番号).is_some() {
            let 理由 = 抽出できなかった理由::Graphiteの生成物の取り込みである {
                モジュール名: 宣言.モジュール名.clone()
            };
            return 宣言の行き先::明示して除外する(抽出できなかった行::生成する(パス, 宣言.行番号, 理由));
        }
        match self.本体の在るファイル(宣言) {
            Some(子) => 宣言の行き先::子を辿る(子),
            None => {
                let 構文 = 保証範囲の外の構文::本番のモジュールを辿れない;
                宣言の行き先::辿れない(抽出できなかった行::生成する(パス, 宣言.行番号, 抽出できなかった理由::保証範囲の外の構文である { 構文 }))
            }
        }
    }

    /// 宣言の本体が在るファイル。既定の探索に任せた宣言は `<名前>.rs` が無ければ `<名前>/mod.rs` を探す。明示した置き場は読み替えない。
    fn 本体の在るファイル(&self, 宣言: &モジュール宣言) -> Option<PathBuf> {
        let 子 = 宣言.対象の物理ファイル()?;
        if self.原文一覧.パスで探す(&子).is_some() {
            return Some(子);
        }
        let 読み替えた子 = 子.with_extension("").join(format!("mod{ソースのファイルの末尾}"));
        (宣言.置き場の指定 == 置き場の指定::既定の探索に任せる && self.原文一覧.パスで探す(&読み替えた子).is_some()).then_some(読み替えた子)
    }
}
