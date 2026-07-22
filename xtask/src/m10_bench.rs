//! M10の表面流とSPHのGPU試作を固定条件で実行し、Vulkan検証結果とパス別GPU時間を得る。

use std::process::{Command, ExitCode};

const フレーム数: &str = "600";

pub fn 実行する() -> ExitCode {
    for モード in ["--surface-flow", "--sph-512", "--sph-1024", "--sph-2048"] {
        if !一条件を実行する(モード) {
            return ExitCode::FAILURE;
        }
    }
    println!("[xtask] m10-bench成功");
    ExitCode::SUCCESS
}

fn 一条件を実行する(モード: &str) -> bool {
    let 引数一覧 = [
        "run",
        "-p",
        "blitz_app",
        "--",
        "--scene",
        "quad",
        "--frames",
        フレーム数,
        モード,
        "--report-gpu-times",
    ];
    println!("[xtask] cargo {} を実行", 引数一覧.join(" "));
    match Command::new("cargo").args(引数一覧).status() {
        Ok(状態) if 状態.success() => true,
        Ok(状態) => {
            eprintln!("[xtask] m10-benchが終了コード{状態}で失敗した");
            false
        }
        Err(誤り) => {
            eprintln!("[xtask] cargoの起動に失敗: {誤り}");
            false
        }
    }
}
