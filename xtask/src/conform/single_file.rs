//! 原文1ファイルだけを見て判定できる検査をまとめて走らせる工程。受け取るのは対象ファイルの一覧、
//! 返すのは違反と対象から外した生成物の報告か、原文を読めなかった理由である。
//!
//! ここに集めるのは、行数の上限・禁止語と絵文字・宣言の間のコメント行・試験ファイルの直置き・取り込みの境界・
//! 不正なallowの緩和・Drop実装の配置・個体群の基盤の抽象名・剛体の単精度3つ組の宣言・二段の位置の口・参照パスの実在である。
//! どれもそのファイルの原文と置き場だけで答えが出るため、拡張子ごとの呼び分けもここで閉じる。
//! 複数のファイルを突き合わせて初めて判定できる検査は `whole_repository.rs` が持つ。
//!
//! 例外が1つある。Graphiteの生成物かどうかは、見出しが名乗る生成元のファイルの原文も読まないと決まらないため、全部の原文を先に読んでから判定する。
//! 生成物と認めたファイルには、行数・宣言の説明の注釈・参照パスの実在の3検査を当てない(2026-09-26のオーナー裁定。Issue #187)。
//! 残りの検査(禁止語と絵文字・不正なallowの緩和など)は生成物にも当てる。

use std::path::{Path, PathBuf};

use super::error::規約検査の破れ;
use super::graphiteのコード::{Graphiteの生成物の一覧, 対象外にした生成物};
use super::report::検査の報告;
use super::violation::違反;
use super::{allow_lint, declaration_comment_line, doc_reference, drop_impl, ecs_abstract_name, forbidden_strings, line_count, module_import_boundary, rigid_raw_triplet, test_directory_layout, two_tier_fold_boundary};

/// 生成物と認めたファイルから外す検査の名前。報告の行がこの名前で外した検査を名乗る。
const 生成物から外す検査: &str = "行数・宣言の説明の注釈・参照パスの実在";

pub fn ファイル単位の検査を行う(ファイル一覧: &[PathBuf]) -> Result<検査の報告, 規約検査の破れ> {
    let mut 原文一覧 = Vec::with_capacity(ファイル一覧.len());
    for パス in ファイル一覧 {
        let 内容 = std::fs::read_to_string(パス).map_err(|誤り| 規約検査の破れ::ファイルを読めなかった(パス, 誤り))?;
        原文一覧.push((パス.clone(), 内容));
    }
    let 生成物 = Graphiteの生成物の一覧::原文一覧から見分ける(&原文一覧);
    let mut 違反一覧 = 生成物.結び付かない見出しの違反一覧();
    for (パス, 内容) in &原文一覧 {
        違反一覧.extend(一ファイルの違反を集める(パス, 内容, &生成物));
    }
    let 外した生成物 = 対象外にした生成物::生成する(生成物から外す検査, 生成物.生成物一覧().to_vec());
    Ok(検査の報告::生成する(違反一覧, Vec::new()).対象外にした生成物を足す(外した生成物))
}

fn 一ファイルの違反を集める(パス: &Path, 内容: &str, 生成物: &Graphiteの生成物の一覧) -> Vec<違反> {
    let graphiteの生成物か = 生成物.生成物か(パス);
    let mut 違反一覧 = Vec::new();
    let 拡張子 = パス.extension().and_then(|拡張子| 拡張子.to_str()).unwrap_or("");
    if 拡張子 == "rs" || 拡張子 == "slang" {
        if !graphiteの生成物か {
            違反一覧.extend(line_count::行数の上限超過を検査する(パス, 内容));
            違反一覧.extend(declaration_comment_line::宣言の間のコメント行を検査する(パス, 内容));
        }
        違反一覧.extend(forbidden_strings::禁止語と絵文字の混入を検査する(パス, 内容));
        違反一覧.extend(test_directory_layout::試験ファイルの直置きを検査する(パス));
    }
    if 拡張子 == "ts" && !line_count::生成ファイルか(パス) {
        違反一覧.extend(line_count::行数の上限超過を検査する(パス, 内容));
    }
    if 拡張子 == "slang" {
        違反一覧.extend(module_import_boundary::取り込みの境界を検査する(パス, 内容));
    }
    if 拡張子 == "rs" {
        違反一覧.extend(allow_lint::不正なallowの緩和を検査する(パス, 内容));
        違反一覧.extend(drop_impl::drop実装の配置を検査する(パス, 内容));
        違反一覧.extend(ecs_abstract_name::個体群の基盤の抽象名を検査する(パス, 内容));
        違反一覧.extend(rigid_raw_triplet::剛体の単精度3つ組宣言を検査する(パス, 内容));
        違反一覧.extend(two_tier_fold_boundary::二段の位置の口を検査する(パス, 内容));
    }
    if !graphiteの生成物か {
        違反一覧.extend(doc_reference::参照パスの実在を検査する(パス, 内容));
    }
    違反一覧
}
