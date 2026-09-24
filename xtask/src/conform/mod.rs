//! 規約適合の機械検査。CLAUDE.mdの規約を文書ではなく検査コードで守らせる。
//! 参照: CLAUDE.md「ファイル・関数の分割」「型安全性」「依存の設計」。

mod allow_lint;
mod cargo_toml_parse;
mod declaration_comment_line;
mod dependency_whitelist;
mod depth_contract;
pub(crate) mod design_ontology;
mod doc_reference;
mod doc_section;
mod drop_impl;
mod duplicate_file_literal;
mod ecs_abstract_name;
pub(crate) mod error;
mod extractable_normal_form;
mod forbidden_strings;
mod free_function_whole_type;
mod lighting_query_declaration;
mod line_count;
mod line_count_allowance;
mod module_import_boundary;
mod particle_reference;
mod reload_without_device_wait;
mod removed_object_uniform;
mod removed_slot_material_set;
mod removed_view_pass_lighting;
mod report;
mod rigid_raw_triplet;
mod sample_bodies_consistency;
mod section_reference;
mod shader_binding;
mod shader_constant;
mod shader_form;
mod shader_uniform_alias;
mod single_file;
mod single_lighting_slot_write;
pub(crate) mod source_lexing;
mod split_debt;
mod test_directory_layout;
mod test_file;
mod two_tier_fold_boundary;
mod type_metrics_ledger;
mod verify_output_place;
mod violation;
mod warning;
mod whole_repository;
mod wording_contract;
mod workspace_dependency_features;

use std::process::ExitCode;

use crate::file_scan;
use error::規約検査の破れ;
use report::検査の報告;

const 検査対象ディレクトリ一覧: [&str; 4] = ["crates", "xtask/src", "shaders", "editor_web/src"];
const 検査対象拡張子一覧: [&str; 4] = ["rs", "slang", "md", "ts"];

pub fn 規約を検査する() -> ExitCode {
    match 全違反を集める() {
        Ok(報告) => 報告.表示して終了コードを返す(),
        Err(破れ) => {
            eprintln!("[xtask] conformの検査を実行できなかった: {破れ}");
            ExitCode::FAILURE
        }
    }
}

/// 走査・ファイル単位・全体の3段を順に回して違反と警告を集める。どの段で検査を実行できなくても同じ破れの型で返るため、
/// 段ごとに終了コードへの写し方を書き分けない。
fn 全違反を集める() -> Result<検査の報告, 規約検査の破れ> {
    let ファイル一覧 = file_scan::対象ファイル一覧を集める(&検査対象ディレクトリ一覧, &検査対象拡張子一覧)?;
    let ファイル単位の報告 = 検査の報告::生成する(single_file::ファイル単位の違反を集める(&ファイル一覧)?, Vec::new());
    Ok(ファイル単位の報告.合わせる(whole_repository::複数ファイルを横断する検査の違反一覧を集める()?))
}
