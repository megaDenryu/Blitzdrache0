//! M11の固定長連続実行。子プロセスのRAM・VRAM推移をWindowsカウンターからテキスト採取する。

mod sample;

use std::process::{Command, ExitCode};
use std::thread;
use std::time::{Duration, Instant};

use crate::fetch_assets;

const フレーム数: &str = "3600";
const 採取間隔: Duration = Duration::from_secs(5);
const 制限時間: Duration = Duration::from_secs(120);

pub fn 実行する() -> ExitCode {
    if fetch_assets::実行する() != ExitCode::SUCCESS {
        eprintln!("[xtask] m11-soakのアセット取得に失敗した");
        return ExitCode::FAILURE;
    }
    println!("[xtask] m11-soak用リリースビルド");
    let Ok(ビルド結果) = Command::new("cargo").args(["build", "--release", "-p", "blitz_app"]).status() else {
        eprintln!("[xtask] cargoの起動に失敗した");
        return ExitCode::FAILURE;
    };
    if !ビルド結果.success() {
        return ExitCode::FAILURE;
    }

    let 引数一覧 = ["--scene", "helmet", "--benchmark-frames", フレーム数, "--particles"];
    let Ok(mut 子) = Command::new("target/release/blitz_app.exe").args(引数一覧).spawn() else {
        eprintln!("[xtask] blitz_appの起動に失敗した");
        return ExitCode::FAILURE;
    };
    let 開始 = Instant::now();
    let mut 標本一覧 = Vec::new();
    println!("経過秒,ワーキングセットMiB,プライベートMiB,専用VRAMMiB");

    let 終了状態 = loop {
        thread::sleep(採取間隔);
        match 子.try_wait() {
            Ok(Some(状態)) => break 状態,
            Ok(None) if 開始.elapsed() < 制限時間 => {
                let 標本 = sample::取得する(子.id(), 開始.elapsed());
                match 子.try_wait() {
                    Ok(Some(状態)) => break 状態,
                    Ok(None) => {
                        if let Some(標本) = 標本 {
                            標本.表示する();
                            標本一覧.push(標本);
                        }
                    }
                    Err(誤り) => {
                        eprintln!("[xtask] 採取後の子プロセス状態取得に失敗: {誤り}");
                        let _ = 子.kill();
                        let _ = 子.wait();
                        return ExitCode::FAILURE;
                    }
                }
            }
            Ok(None) => {
                eprintln!("[xtask] m11-soakが制限時間を超えたため終了する");
                let _ = 子.kill();
                let _ = 子.wait();
                return ExitCode::FAILURE;
            }
            Err(誤り) => {
                eprintln!("[xtask] 子プロセス状態の取得に失敗: {誤り}");
                let _ = 子.kill();
                let _ = 子.wait();
                return ExitCode::FAILURE;
            }
        }
    };

    let 要約成功 = sample::要約を表示する(&標本一覧);
    if 終了状態.success() && 要約成功 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
