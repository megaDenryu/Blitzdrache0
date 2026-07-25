//! OW0の描画対象数スケーリング: 二対象の実機画素判定後、1・10・100対象を同じ固定カメラで計測する。

use std::process::{Command, ExitCode};

const ベンチフレーム数: &str = "360";
const 描画対象数一覧: [&str; 3] = ["1", "10", "100"];

pub fn 実行する() -> ExitCode {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::既定を生成する() {
        return ExitCode::FAILURE;
    }
    println!("[xtask] 二描画対象の実機ピクセル判定を実行");
    if !cargoを実行する(&スモーク引数()) {
        return ExitCode::FAILURE;
    }
    for 描画対象数 in 描画対象数一覧 {
        println!("[xtask] 描画対象数{描画対象数}の固定ベンチを実行");
        if !cargoを実行する(&ベンチ引数(描画対象数)) {
            return ExitCode::FAILURE;
        }
    }
    println!("[xtask] object-bench成功");
    ExitCode::SUCCESS
}

fn スモーク引数() -> Vec<&'static str> {
    vec![
        "run",
        "-p",
        "blitz_app",
        "--",
        "--scene",
        "quad",
        "--asset-root",
        "target/runtime_assets",
        "--frames",
        "2",
        "--object-count",
        "2",
        "--unlit",
        "--no-post",
    ]
}

fn ベンチ引数(描画対象数: &'static str) -> Vec<&'static str> {
    vec![
        "run",
        "--release",
        "-p",
        "blitz_app",
        "--",
        "--scene",
        "quad",
        "--asset-root",
        "target/runtime_assets",
        "--benchmark-frames",
        ベンチフレーム数,
        "--object-count",
        描画対象数,
        "--unlit",
        "--no-post",
        "--report-gpu-times",
        "--report-frame-times",
        "--report-memory",
    ]
}

fn cargoを実行する(引数一覧: &[&str]) -> bool {
    println!("[xtask] cargo {} を実行", 引数一覧.join(" "));
    match Command::new("cargo").args(引数一覧).status() {
        Ok(状態) if 状態.success() => true,
        Ok(状態) => {
            eprintln!("[xtask] 子プロセスが終了コード{状態}で失敗した");
            false
        }
        Err(誤り) => {
            eprintln!("[xtask] cargoの起動に失敗した: {誤り}");
            false
        }
    }
}
