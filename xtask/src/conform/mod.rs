//! 規約適合の機械検査。CLAUDE.mdの規約を文書ではなく検査コードで守らせる。
//! 参照: CLAUDE.md「ファイル・関数の分割」「型安全性」「依存の設計」。

mod allow_lint;
mod cargo_toml_parse;
mod dependency_whitelist;
mod doc_reference;
mod doc_section;
mod drop_impl;
mod forbidden_strings;
mod line_count;
mod particle_reference;
mod removed_object_uniform;
mod section_reference;
mod shader_constant;
mod shader_uniform_alias;
mod split_debt;
mod violation;
mod whole_repository;

use std::path::PathBuf;
use std::process::ExitCode;

use crate::file_scan;
use violation::違反;

const 検査対象ディレクトリ一覧: [&str; 3] = ["crates", "xtask/src", "shaders"];
const 検査対象拡張子一覧: [&str; 3] = ["rs", "slang", "md"];

pub fn 実行する() -> ExitCode {
    let ファイル一覧 = match file_scan::対象ファイル一覧を集める(&検査対象ディレクトリ一覧, &検査対象拡張子一覧) {
        Ok(一覧) => 一覧,
        Err(誤り) => {
            eprintln!("[xtask] conformのファイル走査に失敗した: {誤り}");
            return ExitCode::FAILURE;
        }
    };

    let mut 違反一覧 = match ファイル単位の違反を集める(&ファイル一覧) {
        Ok(一覧) => 一覧,
        Err(誤り) => {
            eprintln!("[xtask] conformのファイル読み取りに失敗した: {誤り}");
            return ExitCode::FAILURE;
        }
    };

    match whole_repository::集める() {
        Ok(全体違反一覧) => 違反一覧.extend(全体違反一覧),
        Err(誤り) => {
            eprintln!("[xtask] conformの{誤り}");
            return ExitCode::FAILURE;
        }
    }

    結果を表示する(違反一覧)
}

fn ファイル単位の違反を集める(ファイル一覧: &[PathBuf]) -> Result<Vec<違反>, String> {
    let mut 違反一覧 = Vec::new();
    for パス in ファイル一覧 {
        let 内容 = std::fs::read_to_string(パス).map_err(|誤り| format!("{}の読み取りに失敗した: {誤り}", パス.display()))?;
        let 拡張子 = パス.extension().and_then(|拡張子| 拡張子.to_str()).unwrap_or("");
        if 拡張子 == "rs" || 拡張子 == "slang" {
            違反一覧.extend(line_count::検査する(パス, &内容));
            違反一覧.extend(forbidden_strings::検査する(パス, &内容));
        }
        if 拡張子 == "rs" {
            違反一覧.extend(allow_lint::検査する(パス, &内容));
            違反一覧.extend(drop_impl::検査する(パス, &内容));
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
