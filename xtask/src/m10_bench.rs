//! M10の表面流とSPHのGPU試作を固定条件で実行し、Vulkan検証結果とパス別GPU時間を得る。
//!
//! 出力を捕まえずに画面へ流すのは、1条件が600フレーム走り、進み具合を人がその場で見るためである。
//! 読む世界の置き場をアプリの既定へ任せるのは、この入口が既定の世界の`quad`だけを描くためである。

use std::process::ExitCode;

use crate::acceptance::{
    アプリの起こし方, アプリの起動指定, 世界を読ませて報告を採る実行環境, 描画フレーム数, 検収の実行名, 検収シーン名
};

const シーン名: 検収シーン名 = 検収シーン名::生成する("quad");
const フレーム数: 描画フレーム数 = 描画フレーム数::生成する(600);

/// 測る4つの試作。それぞれの起動指定の語がそのまま条件の名前になる。
const 試作の起動指定一覧: [&str; 4] = ["--surface-flow", "--sph-512", "--sph-1024", "--sph-2048"];

pub fn 実行する() -> ExitCode {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::既定を生成する() {
        return ExitCode::FAILURE;
    }
    let 実行環境 = 世界を読ませて報告を採る実行環境::世界の置き場をアプリの既定に任せて作る(
        アプリの起こし方::毎回cargoに構築させて起動する,
    );
    for 試作の起動指定 in 試作の起動指定一覧 {
        if !一条件を実行する(&実行環境, 試作の起動指定) {
            return ExitCode::FAILURE;
        }
    }
    println!("[xtask] m10-bench成功");
    ExitCode::SUCCESS
}

fn 一条件を実行する(実行環境: &世界を読ませて報告を採る実行環境, 試作の起動指定: &str) -> bool {
    let 指定 = アプリの起動指定::シーンと枚数を決める(シーン名, フレーム数)
        .選択肢を足す(試作の起動指定)
        .選択肢を足す("--report-gpu-times");
    let 実行名 = match 実行名を組む(試作の起動指定) {
        Ok(実行名) => 実行名,
        Err(誤り) => {
            eprintln!("[xtask] {誤り}");
            return false;
        }
    };
    println!("[xtask] m10-bench実行: {試作の起動指定}");
    match 実行環境.画面へ流したまま走らせる(実行名, &指定) {
        Ok(()) => true,
        Err(誤り) => {
            eprintln!("[xtask] m10-benchが失敗した: {誤り}");
            false
        }
    }
}

/// 起動指定の語から実行を指す名前を組む。先頭のハイフン2つを落とすのは、名前が先頭に区切りを持たないためである。
fn 実行名を組む(試作の起動指定: &str) -> Result<検収の実行名, crate::acceptance::検収エラー> {
    検収の実行名::生成する(試作の起動指定.trim_start_matches('-'))
}
