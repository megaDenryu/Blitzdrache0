//! 大規模世界をtarget配下へ生成し、クリーンな2生成と同一条件の再実行を検収する入口。

mod arguments;

use std::process::ExitCode;

pub(crate) fn 実行する(引数一覧: &[String]) -> ExitCode {
    let 広がり = match arguments::世界の広がりを読む(引数一覧) {
        Ok(広がり) => 広がり,
        Err(理由) => {
            eprintln!("[xtask] large-world-assetsの引数が不正: {理由}");
            return ExitCode::FAILURE;
        }
    };
    match crate::game_fox_tour::map_generation_check::大規模世界を確かめる(crate::fox_tour_map_seed::決定性検収の乱数の種, 広がり)
    {
        Ok(要約) => {
            let 東西 = 広がり.東西チャンク数();
            let 南北 = 広がり.南北チャンク数();
            println!("[xtask] large-world-assets成功: {東西}×{南北}チャンク、{要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] large-world-assets失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}
