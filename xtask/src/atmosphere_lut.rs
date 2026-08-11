//! OW5第7b段の検収入口。透過率のベイク済み画像と多重散乱のベイク済み画像をウィンドウなし実行のGPUで焼いて読み戻し、
//! 代表テクセルのCPU正本との一致・全要素の有限性と非負・透過率の値域・同入力2回の完全一致を判定する。
//!
//! ウィンドウもスワップチェーンも作らないため、GUIを使えない環境でもそのまま走る。判定の対象は
//! `blitz_app`の大気のベイク済み画像報告が出す機械可読な行であり、GPU側の生成シェーダーとCPU側の正本が同じ式であることを見る。
//! シェーダーがCPU正本からずれる欠陥は、この検査だけが捉えられる(ユニットテストはCPU側しか見ない)。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「大気のベイク済み画像方式の設計(第7段で実装する)」

mod heading;
mod judgment;
mod parse;
mod table;
mod thresholds;
mod tolerance_choice;
mod validation_judgment;

use std::process::ExitCode;

use crate::acceptance::{アプリの起こし方, 世界を読まずに報告を採る実行環境, 検収の実行名, 終了時報告};

/// この実行を指す名前。絵は書き出さないが、失敗の文面がどの実行かを名指すために要る。
const 大気の焼き上げの実行名: 検収の実行名 = 検収の実行名::定数から生成する("atmosphere_lut");

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] atmosphere-lut成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] atmosphere-lut失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    let 出力 = 報告を採る()?;
    let 報告 = parse::報告を取り出す(出力.本文())?;
    let 検証の告知 = validation_judgment::検証を検査する(&報告.検証).inspect_err(|_| eprint!("{}", 出力.標準エラーの本文()))?;
    let 判定 = judgment::全項目を検査する(&報告)?;
    table::表を出す(&報告);
    println!("[xtask] {検証の告知}");
    Ok(format!(
        "代表{}テクセルがCPU正本と一致し(最大相対誤差{:.3e}・最大絶対誤差{:.3e})、全要素が有限かつ非負で、透過率が0以上1以下に収まり、同じ入力を2回焼いた結果が完全に一致した。{検証の告知}",
        判定.代表テクセル数, 判定.最大相対誤差, 判定.最大絶対誤差
    ))
}

/// 大気のベイク済み画像を焼かせて報告を採る。validationの指摘は判定の一部としてこの入口が読むため、
/// 実行環境が零件を確かめる口は通さない。
fn 報告を採る() -> Result<終了時報告, String> {
    println!("[xtask] blitz_appの大気のベイク済み画像報告を実行");
    let 実行環境 = 世界を読まずに報告を採る実行環境::作る(アプリの起こし方::毎回cargoに構築させて起動する);
    Ok(実行環境.報告を採る(大気の焼き上げの実行名, &["--report-atmosphere-lut"])?)
}
