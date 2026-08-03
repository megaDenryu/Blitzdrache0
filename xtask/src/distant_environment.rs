//! 順3-Iaの検収入口。遠方環境の立方体画像をvalidation層を有効にしたウィンドウなし実行のGPUで焼いて読み戻し、
//! 鍵の判定・validationの指摘・円盤非二重計上の負の対照・全テクセルの健全性・代表テクセルのCPU正本との一致・
//! 面境界の連続性を判定する。
//!
//! 判定の対象は`blitz_app`の遠方環境報告が出す機械可読な行であり、GPU側の生成シェーダーとCPU側の正本が
//! 同じ式であることを見る。シェーダーがCPU正本からずれる欠陥は、この検査だけが捉えられる
//! (ユニットテストはCPU側しか見ない)。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「検収(機械判定)」

mod judgment;
mod parse;
mod thresholds;

use std::process::{Command, ExitCode};

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] distant-environment成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] distant-environment失敗: {理由}");
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
    let 報告 = parse::報告を取り出す(&出力.標準出力).inspect_err(|_| eprintln!("{}", 出力.標準エラー))?;
    let 判定 = judgment::全項目を検査する(&報告).inspect_err(|_| eprintln!("{}", 出力.標準エラー))?;
    表を出す(&報告);
    Ok(format!(
        "鍵の判定が{}条件すべて期待どおりで、太陽円盤放射輝度だけを変えた対がバイト一致し、全{}テクセルが有限かつ非負で、代表{}テクセルがCPU正本と一致し(最大相対誤差{:.3e})、面境界の最大相対差が{:.3e}だった。{}",
        報告.鍵一覧.len(),
        報告.テクセル数,
        判定.代表テクセル数,
        判定.最大相対誤差,
        報告.面境界の最大相対差,
        判定.検証の告知
    ))
}

/// 判定の根拠になった数を人が読める形で並べる。判定そのものは`judgment`が済ませている。
fn 表を出す(報告: &parse::報告) {
    println!("遠方環境の読み戻し:");
    println!(
        "  テクセル数: {} (非有限{} 負{} 予約が非零{})",
        報告.テクセル数, 報告.非有限, 報告.負, 報告.予約が非零
    );
    println!("  円盤対照の不一致: {}", 報告.円盤対照の不一致);
    println!("  面境界の最大相対差: {:.3e}", 報告.面境界の最大相対差);
    for 行 in &報告.鍵一覧 {
        println!("  鍵 {}: {}", 行.条件, 行.指示);
    }
}

fn 報告を採る() -> Result<実行の出力, String> {
    println!("[xtask] cargo run -p blitz_app -- --report-distant-environment を実行");
    let 出力 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--", "--report-distant-environment"])
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
