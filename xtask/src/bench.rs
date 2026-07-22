//! 評価軸3.4節の最小ベンチ入口: 固定シーン(helmet)+固定カメラ(既定姿勢、入力なし)+
//! 固定フレーム数で`blitz_app`を実行し、パス別GPU時間とCPU側フレーム間隔分布を表示する。
//! スモークと同じ思想(繰り返す検証は資産化)で`cargo xtask bench`として登録する。
//! 参照: `_doc/計画/評価軸.md`「3.4 計測の再現性」。

use std::process::{Command, ExitCode};

use crate::fetch_assets;

const フレーム数: &str = "600";

pub fn 実行する() -> ExitCode {
    println!("[xtask] ベンチ用アセットの取得確認");
    if fetch_assets::実行する() != ExitCode::SUCCESS {
        eprintln!("[xtask] benchのアセット取得に失敗した");
        return ExitCode::FAILURE;
    }

    let 引数一覧 = [
        "run",
        "-p",
        "blitz_app",
        "--",
        "--scene",
        "helmet",
        "--frames",
        フレーム数,
        "--particles",
        "--report-gpu-times",
        "--report-frame-times",
    ];
    println!("[xtask] cargo {} を実行", 引数一覧.join(" "));
    match Command::new("cargo").args(引数一覧).status() {
        Ok(状態) if 状態.success() => {
            println!("[xtask] bench成功");
            ExitCode::SUCCESS
        }
        Ok(状態) => {
            eprintln!("[xtask] benchが終了コード{状態}で失敗した");
            ExitCode::FAILURE
        }
        Err(起動誤り) => {
            eprintln!("[xtask] cargoの起動に失敗: {起動誤り}");
            ExitCode::FAILURE
        }
    }
}
