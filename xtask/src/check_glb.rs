//! glTF入力契約の検査の入口。実体はアセットコンパイラの検査器exampleであり、ここが担当するのは引数の受け渡しと
//! 終了コードの写しだけである。起こし方は`asset_generator`の器が持つ。
//! 参照: CLAUDE.md「ツールとドキュメントの配置」(xtaskはツールの唯一の入口)。

use std::process::ExitCode;

use blitz_asset_compiler::検査する契約;

use crate::asset_generator::{アセット生成器の起動, 検査するファイル一覧, 生成の指定, 生成器エラー};

pub fn 実行する(引数一覧: &[String]) -> ExitCode {
    match 検査器を走らせる(引数一覧) {
        Ok(()) => ExitCode::SUCCESS,
        Err(理由) => {
            eprintln!("[xtask] glb契約検査が不合格または失敗である: {理由}");
            eprintln!("使い方: cargo xtask check-glb [--archetype] <検査するglbまたはgltfのパス> ...");
            ExitCode::FAILURE
        }
    }
}

/// 契約を選ぶ綴りは全パスへ同じに効く。パスごとに契約を変えられる形にすると、検査していない契約のファイルが並びの中に紛れる。
fn 検査器を走らせる(引数一覧: &[String]) -> Result<(), 生成器エラー> {
    let (契約, パス一覧) = 検査する契約::引数一覧から取り出す(引数一覧);
    アセット生成器の起動::始める(&生成の指定::入力するglTFの契約を検査する {
        検査する契約: 契約,
        検査するファイル一覧: &検査するファイル一覧::コマンド行の語から生成する(&パス一覧)?,
    })?
    .画面へ流したまま走らせて終わりを待つ()
}
