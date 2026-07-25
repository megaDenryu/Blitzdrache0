//! M11の固定長連続実行。子プロセスのRAM・VRAM推移をWindowsカウンターからテキスト採取する。

use std::process::ExitCode;
use std::time::Duration;

use crate::fetch_assets;
use crate::memory_sampling::{実行しながら採取する, 採取条件};

const フレーム数: &str = "3600";
const 採取間隔: Duration = Duration::from_secs(5);
const 制限時間: Duration = Duration::from_secs(120);

pub fn 実行する() -> ExitCode {
    if fetch_assets::実行する() != ExitCode::SUCCESS {
        eprintln!("[xtask] m11-soakのアセット取得に失敗した");
        return ExitCode::FAILURE;
    }
    if !crate::compile_assets::既定を生成する() {
        return ExitCode::FAILURE;
    }
    if !crate::release_build::実行する("m11-soak") {
        return ExitCode::FAILURE;
    }

    let 条件 = 採取条件 {
        実行ファイル: "target/release/blitz_app.exe",
        引数一覧: &["--scene", "helmet", "--benchmark-frames", フレーム数, "--particles"],
        採取間隔,
        制限時間,
    };
    if 実行しながら採取する(&条件) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
