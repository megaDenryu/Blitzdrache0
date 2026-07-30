//! OW5第7b段の検収入口。透過率のベイク済み画像と多重散乱のベイク済み画像をヘッドレスのGPUで焼いて読み戻し、
//! 代表テクセルのCPU正本との一致・全要素の有限性と非負・透過率の値域・同入力2回の完全一致を判定する。
//!
//! ウィンドウもスワップチェーンも作らないため、GUIを使えない環境でもそのまま走る。判定の対象は
//! `blitz_app`の大気のベイク済み画像報告が出す機械可読な行であり、GPU側の生成シェーダーとCPU側の正本が同じ式であることを見る。
//! シェーダーがCPU正本からずれる欠陥は、この検査だけが捉えられる(ユニットテストはCPU側しか見ない)。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「大気のベイク済み画像方式の設計(第7段で実装する)」

mod judgment;
mod parse;
mod table;
mod thresholds;
mod validation_judgment;

use std::process::{Command, ExitCode};

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

/// blitz_appが書いた標準出力と標準エラー。validationの指摘は標準エラーへ出るため、
/// 検収が落ちたときに読めるよう成功時も保持する。
struct 実行の出力 {
    標準出力: String,
    標準エラー: String,
}

fn 検収する() -> Result<String, String> {
    let 出力 = 報告を採る()?;
    let 報告 = parse::報告を取り出す(&出力.標準出力)?;
    let 検証の告知 = validation_judgment::検証を検査する(&報告.検証).inspect_err(|_| eprintln!("{}", 出力.標準エラー))?;
    let 判定 = judgment::全項目を検査する(&報告)?;
    table::表を出す(&報告);
    println!("[xtask] {検証の告知}");
    Ok(format!(
        "代表{}テクセルがCPU正本と一致し(最大相対誤差{:.3e}・最大絶対誤差{:.3e})、全要素が有限かつ非負で、透過率が0以上1以下に収まり、同じ入力を2回焼いた結果が完全に一致した。{検証の告知}",
        判定.代表テクセル数, 判定.最大相対誤差, 判定.最大絶対誤差
    ))
}

fn 報告を採る() -> Result<実行の出力, String> {
    println!("[xtask] cargo run -p blitz_app -- --report-atmosphere-lut を実行");
    let 出力 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--", "--report-atmosphere-lut"])
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった: {誤り}"))?;
    let 標準エラー = String::from_utf8_lossy(&出力.stderr).into_owned();
    if !出力.status.success() {
        return Err(format!("blitz_appが{}で失敗した: {標準エラー}", 出力.status));
    }
    Ok(実行の出力 {
        標準出力: String::from_utf8_lossy(&出力.stdout).into_owned(),
        標準エラー,
    })
}
