//! M11の固定長連続実行。子プロセスのRAM・VRAM推移をWindowsカウンターからテキスト採取する。

use std::process::ExitCode;
use std::time::Duration;

use crate::fetch_assets;
use crate::memory_sampling::{実行しながら採取する, 採取条件};

const フレーム数: &str = "3600";
const 採取間隔: Duration = Duration::from_secs(5);
const 制限時間: Duration = Duration::from_secs(120);

pub fn 連続実行のメモリ推移を計測する() -> ExitCode {
    if fetch_assets::標準サンプルを取得する() != ExitCode::SUCCESS {
        eprintln!("[xtask] m11-soakのアセット取得に失敗した");
        return ExitCode::FAILURE;
    }
    if !crate::compile_assets::既定を生成する() {
        return ExitCode::FAILURE;
    }
    if !crate::release_build::構築して合否を返す("m11-soak") {
        return ExitCode::FAILURE;
    }

    let 条件 = 採取条件 {
        起こし方: crate::acceptance::アプリの起こし方::構築済みのリリース版を直に起動する,
        引数一覧: &["--scene", "helmet", "--benchmark-frames", フレーム数, "--particles"],
        採取間隔,
        制限時間,
        標準出力先: None,
    };
    if 実行しながら採取する(&条件).is_some() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
