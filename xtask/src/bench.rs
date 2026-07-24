//! 評価軸3.4節の最小ベンチ入口: 固定シーン(helmet)+固定カメラ(既定姿勢、入力なし)+
//! 固定フレーム数で`blitz_app`を実行し、パス別GPU時間とCPU側フレーム間隔分布を表示する。
//! スモークと同じ思想(繰り返す検証は資産化)で`cargo xtask bench`として登録する。
//! 参照: `_doc/計画/評価軸.md`「3.4 計測の再現性」。

use std::process::{Command, ExitCode};

use crate::fetch_assets;

const フレーム数: &str = "600";

/// 実表示計測を有効にするかどうか。2つの計測条件を取り違えないよう、真偽値でなく型で持つ。
///
/// 注意: `あり`は`vkWaitForPresentKHR`が表示まで描画ループを止める条件であり、拡張の有効化も伴う。
/// 既存の性能時系列(M11以降)と比較する値は`なし`で採る。両条件の比較は交互実行で行うこと。
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum 実表示計測 {
    なし,
    あり,
}

pub fn 実行する() -> ExitCode {
    条件を指定して実行する(実表示計測::なし)
}

pub fn 実表示計測つきで実行する() -> ExitCode {
    println!("[xtask] 注意: 実表示計測は表示まで描画ループを止めるため、フレームペーシングを変えうる条件である");
    println!("[xtask] 注意: 既存の性能時系列と比較する値は`cargo xtask bench`で採ること");
    条件を指定して実行する(実表示計測::あり)
}

fn 条件を指定して実行する(実表示計測: 実表示計測) -> ExitCode {
    println!("[xtask] ベンチ用アセットの取得確認");
    if fetch_assets::実行する() != ExitCode::SUCCESS {
        eprintln!("[xtask] benchのアセット取得に失敗した");
        return ExitCode::FAILURE;
    }
    if !crate::compile_assets::既定を生成する() {
        return ExitCode::FAILURE;
    }

    let mut 引数一覧 = vec![
        "run",
        "--release",
        "-p",
        "blitz_app",
        "--",
        "--scene",
        "helmet",
        "--benchmark-frames",
        フレーム数,
        "--particles",
        "--report-gpu-times",
        "--report-frame-times",
        "--report-memory",
    ];
    if 実表示計測 == 実表示計測::あり {
        引数一覧.push("--report-display-timing");
    }
    println!("[xtask] cargo {} を実行", 引数一覧.join(" "));
    子プロセスを実行する(&引数一覧)
}

fn 子プロセスを実行する(引数一覧: &[&str]) -> ExitCode {
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
