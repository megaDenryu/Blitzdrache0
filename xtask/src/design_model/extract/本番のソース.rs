//! クレートの入口から本番のモジュール宣言を辿り、抽出規則が共有するソース一覧を組む。
//! 試験用の原文では入口を明示して与える。本番の走査はlib.rsとmain.rsだけを入口にする。

use super::ontology_scope::適用範囲のクレートのパスか;
use super::本番の行::本番の行を選ぶ;
use super::{保証範囲の外の構文, 抽出できなかった理由, 抽出できなかった行};
use crate::conform::design_ontology::module_declaration::置き場の指定;
use crate::conform::design_ontology::module_declaration_extract::モジュール宣言の抽出;
use crate::conform::design_ontology::module_path::ソースのファイルの末尾;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct 本番のソース {
    pub 行一覧: Vec<(PathBuf, Vec<String>)>,
    pub 欠落一覧: Vec<抽出できなかった行>,
}

impl 本番のソース {
    pub fn 入口から集める(原文一覧: Vec<(PathBuf, String)>, 入口一覧: Vec<PathBuf>) -> Self {
        let 原文一覧: Vec<_> = 原文一覧.into_iter().filter(|(パス, _)| 適用範囲のクレートのパスか(パス)).collect();
        let mut 待ち = 入口一覧;
        let mut 済み = HashSet::new();
        let mut 本番 = Self {
            行一覧: Vec::new(), 欠落一覧: Vec::new()
        };
        while let Some(パス) = 待ち.pop() {
            if !済み.insert(パス.clone()) {
                continue;
            }
            let Some((_, 原文)) = 原文一覧.iter().find(|(候補, _)| *候補 == パス) else {
                continue;
            };
            let (mut 行一覧, 欠落) = 本番の行を選ぶ(&パス, 原文);
            let mut 子一覧 = Vec::new();
            let mut モジュールの欠落 = Vec::new();
            for 宣言 in モジュール宣言の抽出::生成する(パス.clone(), 原文).宣言一覧() {
                if 行一覧.get(宣言.行番号 - 1).is_none_or(|行| 行.trim().is_empty()) {
                    continue;
                }
                if let Some(子) = 宣言.対象の物理ファイル() {
                    let 候補 = if 原文一覧.iter().any(|(パス, _)| *パス == 子) {
                        Some(子)
                    } else if 宣言.置き場の指定 == 置き場の指定::既定の探索に任せる {
                        Some(子.with_extension("").join(format!("mod{ソースのファイルの末尾}")))
                    } else {
                        None
                    };
                    if let Some(候補) = 候補.filter(|候補| 原文一覧.iter().any(|(パス, _)| パス == 候補)) {
                        子一覧.push(候補);
                        continue;
                    }
                }
                モジュールの欠落.push(抽出できなかった行::生成する(
                    &パス,
                    宣言.行番号,
                    抽出できなかった理由::保証範囲の外の構文である {
                        構文: 保証範囲の外の構文::本番のモジュールを辿れない,
                    },
                ));
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
