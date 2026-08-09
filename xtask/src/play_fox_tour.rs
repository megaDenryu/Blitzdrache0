//! クソゲー1本目「キツネの場所巡り」を実際に遊ぶ入口。担当するのは、これから遊ぶマップの乱数の種を知らせることと、
//! 遊ぶのに要る起動指定を積んでblitz_appを無期限実行で起動することである。世界を開く指定は`fox_tour_launch`が、
//! 種の読み出しは`fox_tour_map_seed`が持つ。
//!
//! この入口を置くのは、遊ぶのに5つの起動指定が要り、それを手で並べると検収が通った条件と違う条件で遊ぶことに
//! なるためである。マップが未生成の環境では先に`cargo xtask gen-game-map --seed <数>`を実行する。

use std::process::{Command, ExitCode};

use crate::fox_tour_map_seed::乱数の種の読み取り結果;

pub fn 実行する() -> ExitCode {
    let mut コマンド = Command::new("cargo");
    コマンド.args(["run", "-p", "blitz_app", "--release", "--"]);
    crate::fox_tour_launch::世界を開く指定を積む(&mut コマンド);
    コマンド.args(["--game", "fox_tour"]);
    これから遊ぶマップの乱数の種を知らせる();
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

/// どの種の生成物を遊ぶのかを起動の前に出す。`cargo xtask game-fox-tour`が検収の種でマップを作り直すため、
/// 別の種で生成した後に検収を走らせるとマップが黙って入れ替わる。その入れ替わりに気づく手がかりとして出す。
fn これから遊ぶマップの乱数の種を知らせる() {
    match crate::fox_tour_map_seed::場所巡りの世界の生成に使った乱数の種を読む() {
        乱数の種の読み取り結果::読めた種(種) => {
            println!("[xtask] これから遊ぶマップは乱数の種{種}から生成したものである");
        }
        乱数の種の読み取り結果::読めなかった事情(事情) => {
            println!("[xtask] これから遊ぶマップの乱数の種が分からない({事情})");
            println!("[xtask] 種をはっきりさせるには cargo xtask gen-game-map --seed <32ビットの非負整数> でマップを作り直す");
        }
    }
}
