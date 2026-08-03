//! 順3-Ibの検収入口。与えた遠方環境の上に拡散照度・鏡面畳込み・反射率積分表をvalidation層を有効にした
//! ウィンドウなし実行のGPUで焼いて読み戻し、定数環境の解析解との一致・全テクセルの健全性・粗さを上げたときの
//! ばらつきの単調な減り・最詳細段が遠方環境の複製であること・面境界の連続性・代表テクセルのCPU正本との一致を判定する。
//!
//! 遠方環境の検収と別のコマンドにするのは、判定の関心事が違うためである。あちらは天空放射輝度供給から焼いた1枚が
//! 大気の数学と一致するかを見るのに対し、こちらは与えた環境と畳み込みの結果の関係だけを見て、大気を1つも使わない。
//! 1つのコマンドへ混ぜると、大気の欠陥と畳み込みの欠陥が同じ失敗として出る。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「検収(機械判定)」

mod judgment;
mod parse;
mod rows;
mod thresholds;

use std::process::{Command, ExitCode};

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] derived-environment成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] derived-environment失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    let (標準出力, 標準エラー) = 報告を採る()?;
    let 報告 = parse::報告を取り出す(&標準出力).inspect_err(|_| eprintln!("{標準エラー}"))?;
    let 判定 = judgment::全項目を検査する(&報告).inspect_err(|_| eprintln!("{標準エラー}"))?;
    表を出す(&報告);
    Ok(要約を組む(&報告, &判定))
}

fn 要約を組む(報告: &rows::報告, 判定: &judgment::判定) -> String {
    format!(
        "定数環境で拡散照度が円周率×放射輝度・鏡面畳込みが全粗さ段とも放射輝度そのものになり(最大相対誤差{:.3e})、粗さを上げるほど{}段のばらつきが単調に減り、最詳細段が遠方環境とバイト一致し、面境界の相対差に一辺を掛けた値の最大が{:.3}で、代表{}テクセルがCPU正本と一致した(最大相対誤差{:.3e})。{}",
        判定.定数環境の最大相対誤差,
        報告.分散.len(),
        判定.面境界の角あたりの最大変化,
        判定.代表テクセル数,
        判定.最大相対誤差,
        判定.検証の告知
    )
}

/// 判定の根拠になった数を人が読める形で並べる。判定そのものは`judgment`が済ませている。
fn 表を出す(報告: &rows::報告) {
    println!("派生表現の読み戻し:");
    for 行 in &報告.統計 {
        println!(
            "  {}: テクセル数{} (非有限{} 負{} 予約が非零{})",
            行.種別, 行.テクセル数, 行.非有限, 行.負, 行.予約が非零
        );
    }
    for 行 in &報告.分散 {
        println!("  鏡面段{}のばらつき: {:.3e}", 行.段, 行.分散);
    }
    for 行 in &報告.面境界 {
        println!(
            "  面境界({} 段{} 一辺{}): 稜線{}本 最大相対差{:.3e}",
            行.種別, 行.段, 行.一辺, 行.稜線数, 行.最大相対差
        );
    }
}

fn 報告を採る() -> Result<(String, String), String> {
    println!("[xtask] cargo run -p blitz_app -- --report-derived-environment を実行");
    let 出力 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--", "--report-derived-environment"])
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった: {誤り}"))?;
    let 標準エラー = String::from_utf8_lossy(&出力.stderr).into_owned();
    if !出力.status.success() {
        return Err(format!("blitz_appが{}で失敗した: {標準エラー}", 出力.status));
    }
    Ok((String::from_utf8_lossy(&出力.stdout).into_owned(), 標準エラー))
}
