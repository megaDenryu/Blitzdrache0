//! 検証用ソースアセットの生成。実体はアセットコンパイラの生成器exampleであり、その起こし方は`asset_generator`が持つ。
//! 参照: CLAUDE.md「ツールとドキュメントの配置」(xtaskはツールの唯一の入口)。

use std::process::ExitCode;

use crate::asset_generator::{アセット生成器の起動, 生成の指定, 生成器エラー};

pub fn 実行する() -> ExitCode {
    if 生成する() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}

pub fn 生成する() -> bool {
    println!("[xtask] 検証用ソースアセットの生成器を実行");
    match 生成器を走らせる() {
        Ok(()) => {
            println!("[xtask] ソースアセット生成成功");
            true
        }
        Err(理由) => {
            eprintln!("[xtask] ソースアセット生成に失敗した: {理由}");
            false
        }
    }
}

fn 生成器を走らせる() -> Result<(), 生成器エラー> {
    アセット生成器の起動::始める(&生成の指定::ソースアセットを生成する {
        場所巡りの世界の種: None,
        ソースルート: None,
        世界の広がり: None,
    })?
    .画面へ流したまま走らせて終わりを待つ()
}
