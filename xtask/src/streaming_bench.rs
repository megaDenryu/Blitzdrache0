//! OW2のDoD計測入口。固定経路を自動移動させ、予算を十分に取った条件と縮退させる条件の2つでチャンクの読込・解除を測る。
//! 1つの条件では「反復してもRAM・VRAMが増えない」と「縮退の順序が再現する」の両方を確かめられないため、条件を分けて順に実行する。
//! 参照: `_doc/設計/チャンクストリーミング.md`「固定経路の自動移動」

mod condition_a;
mod condition_b;
mod order_compare;
mod order_sequence;

use std::process::ExitCode;

const 既定条件Aフレーム数: &str = "3600";

pub fn ストリーミング経路の資源を計測する(引数一覧: &[String]) -> ExitCode {
    let フレーム数 = match 引数一覧 {
        [] => 既定条件Aフレーム数.to_string(),
        [フレーム数] if フレーム数.parse::<u32>().is_ok() => フレーム数.clone(),
        _ => {
            eprintln!("使い方: cargo xtask streaming-bench [条件Aのフレーム数]");
            return ExitCode::FAILURE;
        }
    };
    if !crate::gen_source_assets::検証用ソースアセットを生成して成否を返す() || !crate::compile_assets::既定を生成する() {
        return ExitCode::FAILURE;
    }
    if !condition_a::固定経路を反復実行しながら資源を採取する(&フレーム数) {
        eprintln!("[xtask] streaming-bench失敗: 条件A(予算を十分に取った反復)");
        return ExitCode::FAILURE;
    }
    if !condition_b::固定経路を二回実行して読込解除順を突き合わせる() {
        eprintln!("[xtask] streaming-bench失敗: 条件B(縮退させた順序の再現)");
        return ExitCode::FAILURE;
    }
    println!("[xtask] streaming-bench成功");
    ExitCode::SUCCESS
}
