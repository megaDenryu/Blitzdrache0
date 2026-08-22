//! OW5第1段の検収入口。時刻から天空状態を導く純関数を本番の呼び出し元(blitz_appの天空状態報告)越しに走らせ、
//! 代表時刻の導出表を出したうえで、決定性・日境界の循環・太陽方向の単位長・全出力の有限値・
//! 正午の高度が最大で真夜中が最小であることを判定する。描画を伴わないためGPUもウィンドウも要らない。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「検証の設計」

mod altitude_check;
mod error;
mod judgment;
mod parse;
mod row_check;
mod series_check;
#[cfg(test)]
mod series_check_tests;
mod table;

use std::process::ExitCode;

use error::天空状態の検収エラー;

use crate::acceptance::{
    アプリの起こし方, 世界を読まずに報告を採る実行環境, 検収の実行名, 検収エラー, 終了時報告
};

/// この実行を指す名前。絵は書き出さないが、失敗の文面がどの実行かを名指すために要る。
const 天空状態の実行名: 検収の実行名 = 検収の実行名::定数から生成する("sky_state");

pub fn 天空状態の導出を確認する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] sky-state成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] sky-state失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, 天空状態の検収エラー> {
    let 報告 = 報告を採る()?;
    let 行一覧 = parse::行一覧を取り出す(&報告)?;
    judgment::全項目を検査する(&行一覧)?;
    table::表を出す(&行一覧)?;
    let 代表時刻数 = 行一覧.iter().filter(|行| 行.系列 == "初回").count();
    Ok(format!(
        "代表{代表時刻数}時刻の{}行について、2回の導出の完全一致・前日と翌日の循環・太陽方向の単位長・全出力の有限値・正午の最大と真夜中の最小を確かめた",
        行一覧.len()
    ))
}

fn 報告を採る() -> Result<終了時報告, 検収エラー> {
    println!("[xtask] blitz_appの天空状態報告を実行");
    let 実行環境 = 世界を読まずに報告を採る実行環境::作る(アプリの起こし方::毎回cargoに構築させて起動する);
    実行環境.報告を採る(天空状態の実行名, &["--report-sky-state"])
}
