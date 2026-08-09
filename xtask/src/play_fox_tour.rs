//! クソゲー1本目「キツネの場所巡り」を実際に遊ぶ入口。担当するのは、遊ぶのに要る起動指定を積んでblitz_appを
//! 無期限実行で起動することだけである。世界を開く指定は`fox_tour_launch`が持つ。
//!
//! この入口を置くのは、遊ぶのに5つの起動指定が要り、それを手で並べると検収が通った条件と違う条件で遊ぶことに
//! なるためである。マップが未生成の環境では先に`cargo xtask gen-game-map --seed <数>`を実行する。

use std::process::{Command, ExitCode};

pub fn 実行する() -> ExitCode {
    let mut コマンド = Command::new("cargo");
    コマンド.args(["run", "-p", "blitz_app", "--release", "--"]);
    crate::fox_tour_launch::世界を開く指定を積む(&mut コマンド);
    コマンド.args(["--game", "fox_tour"]);
    println!("[xtask] キツネの場所巡りを起動する(Enterではじめる、矢印キーで歩く、Escで終了確認)");
    match コマンド.status() {
        Ok(終了状態) if 終了状態.success() => ExitCode::SUCCESS,
        Ok(終了状態) => {
            eprintln!("[xtask] blitz_appが{終了状態}で終わった");
            ExitCode::FAILURE
        }
        Err(誤り) => {
            eprintln!("[xtask] cargoの起動に失敗: {誤り}");
            ExitCode::FAILURE
        }
    }
}
