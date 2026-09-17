//! 原文1ファイルだけを見て判定できる検査をまとめて走らせる工程。受け取るのは対象ファイルの一覧、
//! 返すのは違反一覧か、原文を読めなかった理由である。
//!
//! ここに集めるのは、行数の上限・禁止語と絵文字・宣言の間のコメント行・試験ファイルの直置き・取り込みの境界・
//! 不正なallowの緩和・Drop実装の配置・個体群の基盤の抽象名・剛体の単精度3つ組の宣言・二段の位置の口・参照パスの実在である。
//! どれもそのファイルの原文と置き場だけで答えが出るため、拡張子ごとの呼び分けもここで閉じる。
//! 複数のファイルを突き合わせて初めて判定できる検査は `whole_repository.rs` が持つ。

use std::path::PathBuf;

use super::error::規約検査の破れ;
use super::violation::違反;
use super::{allow_lint, declaration_comment_line, doc_reference, drop_impl, ecs_abstract_name, forbidden_strings, line_count, module_import_boundary, rigid_raw_triplet, test_directory_layout, two_tier_fold_boundary};

pub fn ファイル単位の違反を集める(ファイル一覧: &[PathBuf]) -> Result<Vec<違反>, 規約検査の破れ> {
    let mut 違反一覧 = Vec::new();
    for パス in ファイル一覧 {
        let 内容 = std::fs::read_to_string(パス).map_err(|誤り| 規約検査の破れ::ファイルを読めなかった(パス, 誤り))?;
        let 拡張子 = パス.extension().and_then(|拡張子| 拡張子.to_str()).unwrap_or("");
        if 拡張子 == "rs" || 拡張子 == "slang" {
            違反一覧.extend(line_count::行数の上限超過を検査する(パス, &内容));
            違反一覧.extend(forbidden_strings::禁止語と絵文字の混入を検査する(パス, &内容));
            違反一覧.extend(declaration_comment_line::宣言の間のコメント行を検査する(パス, &内容));
            違反一覧.extend(test_directory_layout::試験ファイルの直置きを検査する(パス));
        }
        if 拡張子 == "ts" && !line_count::生成ファイルか(パス) {
            違反一覧.extend(line_count::行数の上限超過を検査する(パス, &内容));
        }
        if 拡張子 == "slang" {
            違反一覧.extend(module_import_boundary::取り込みの境界を検査する(パス, &内容));
        }
        if 拡張子 == "rs" {
            違反一覧.extend(allow_lint::不正なallowの緩和を検査する(パス, &内容));
            違反一覧.extend(drop_impl::drop実装の配置を検査する(パス, &内容));
            違反一覧.extend(ecs_abstract_name::個体群の基盤の抽象名を検査する(パス, &内容));
            違反一覧.extend(rigid_raw_triplet::剛体の単精度3つ組宣言を検査する(パス, &内容));
            違反一覧.extend(two_tier_fold_boundary::二段の位置の口を検査する(パス, &内容));
        }
        違反一覧.extend(doc_reference::参照パスの実在を検査する(パス, &内容));
    }
    Ok(違反一覧)
}
