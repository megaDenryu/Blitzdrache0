//! 規約適合の機械検査。CLAUDE.mdの規約を文書ではなく検査コードで守らせる。
//! 参照: CLAUDE.md「ファイル・関数の分割」「型安全性」「依存の設計」。

mod allow_lint;
mod cargo_toml_parse;
mod declaration_comment_line;
mod dependency_whitelist;
mod depth_contract;
mod doc_reference;
mod doc_section;
mod drop_impl;
mod duplicate_file_literal;
mod error;
mod forbidden_strings;
mod lighting_query_declaration;
mod line_count;
mod module_import_boundary;
mod particle_reference;
mod reload_without_device_wait;
mod removed_object_uniform;
mod removed_slot_material_set;
mod removed_view_pass_lighting;
mod rigid_raw_triplet;
mod sample_bodies_consistency;
mod section_reference;
mod shader_binding;
mod shader_constant;
mod shader_form;
mod shader_uniform_alias;
mod single_lighting_slot_write;
mod source_lexing;
mod split_debt;
mod violation;
mod whole_repository;
mod wording_contract;
mod workspace_dependency_features;

use std::path::PathBuf;
use std::process::ExitCode;

use crate::file_scan;
use error::規約検査の破れ;
use violation::違反;

const 検査対象ディレクトリ一覧: [&str; 4] = ["crates", "xtask/src", "shaders", "editor_web/src"];
const 検査対象拡張子一覧: [&str; 4] = ["rs", "slang", "md", "ts"];

pub fn 規約を検査する() -> ExitCode {
    match 全違反を集める() {
        Ok(違反一覧) => 結果を表示する(違反一覧),
        Err(破れ) => {
            eprintln!("[xtask] conformの検査を実行できなかった: {破れ}");
            ExitCode::FAILURE
        }
    }
}

/// 走査・ファイル単位・全体の3段を順に回して違反を集める。どの段で検査を実行できなくても同じ破れの型で返るため、
/// 段ごとに終了コードへの写し方を書き分けない。
fn 全違反を集める() -> Result<Vec<違反>, 規約検査の破れ> {
    let ファイル一覧 = file_scan::対象ファイル一覧を集める(&検査対象ディレクトリ一覧, &検査対象拡張子一覧)?;
    let mut 違反一覧 = ファイル単位の違反を集める(&ファイル一覧)?;
    違反一覧.extend(whole_repository::集める()?);
    Ok(違反一覧)
}

fn ファイル単位の違反を集める(ファイル一覧: &[PathBuf]) -> Result<Vec<違反>, 規約検査の破れ> {
    let mut 違反一覧 = Vec::new();
    for パス in ファイル一覧 {
        let 内容 = std::fs::read_to_string(パス).map_err(|誤り| 規約検査の破れ::ファイルを読めなかった(パス, 誤り))?;
        let 拡張子 = パス.extension().and_then(|拡張子| 拡張子.to_str()).unwrap_or("");
        if 拡張子 == "rs" || 拡張子 == "slang" {
            違反一覧.extend(line_count::検査する(パス, &内容));
            違反一覧.extend(forbidden_strings::検査する(パス, &内容));
            違反一覧.extend(declaration_comment_line::検査する(パス, &内容));
        }
        if 拡張子 == "ts" && !line_count::生成ファイルか(パス) {
            違反一覧.extend(line_count::検査する(パス, &内容));
        }
        if 拡張子 == "slang" {
            違反一覧.extend(module_import_boundary::検査する(パス, &内容));
        }
        if 拡張子 == "rs" {
            違反一覧.extend(allow_lint::検査する(パス, &内容));
            違反一覧.extend(drop_impl::検査する(パス, &内容));
            違反一覧.extend(rigid_raw_triplet::検査する(パス, &内容));
        }
        違反一覧.extend(doc_reference::検査する(パス, &内容));
    }
    Ok(違反一覧)
}

fn 結果を表示する(違反一覧: Vec<違反>) -> ExitCode {
    if 違反一覧.is_empty() {
        println!("[xtask] conform成功: 違反0件");
        return ExitCode::SUCCESS;
    }
    println!("[xtask] conform失敗: 違反{}件", 違反一覧.len());
    for 違反 in &違反一覧 {
        println!("{違反}");
    }
    ExitCode::FAILURE
}
