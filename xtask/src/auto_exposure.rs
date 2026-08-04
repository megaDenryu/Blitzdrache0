//! 順3-IIb(自動露出)の検収入口。担当するのは、目視見本の庭を本番の描画経路で描いた実行から
//! 「CPU正本とGPUの写しが同じヒストグラムと同じ目標補正段を出したか」と「同じ入力から同じ適応系列が出るか」を
//! 機械判定することである。
//!
//! CPU正本との突き合わせそのものはblitz_appが行い、この入口はその報告の行を読むだけである。
//! 正本(`crates/blitz_engine/src/auto_exposure/`)を読めるのはコンポジションルートだけであり、
//! 外部依存を持たないこの入口が正本の写しを持つと、その写しの正しさが検収の前提になってしまう。
//!
//! 決定性を同じ条件の2実行で見るのは、自動露出の経過秒が実時間でなく固定の刻みだからである
//! (`crates/blitz_app/src/app/time_of_day/wiring/exposure.rs`)。刻みが実時間なら系列は実行ごとに変わる。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「順3-IIの実装設計」

mod judgment;
mod parse;
mod run;

use std::path::PathBuf;
use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/auto_exposure";
/// 判定に使う時刻。正午は明部が上寄りで画面全体の分布が広く、集計と導出の食い違いが最も出やすい。
const 正午の一日内秒: &str = "43200";

pub(crate) fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] auto-exposure成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] auto-exposure失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    crate::visual_sample_world::用意する()?;
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;

    let 一回目 = run::描画して報告を読む(&出力先, "noon_first", 正午の一日内秒)?;
    let 二回目 = run::描画して報告を読む(&出力先, "noon_second", 正午の一日内秒)?;
    judgment::一致を判定する(&一回目)?;
    judgment::二実行の一致を判定する(&一回目, &二回目)?;
    Ok(judgment::要約を組む(&一回目, &出力先))
}
