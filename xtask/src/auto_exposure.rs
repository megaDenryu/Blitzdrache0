//! 順3-IIb(自動露出)の検収入口。担当するのは、目視見本の庭を本番の描画経路で描いた実行から
//! 「CPU正本とGPUの写しが同じヒストグラムと同じ目標補正段を出したか」と「同じ入力から同じ適応系列が出るか」を
//! 機械判定することである。
//!
//! CPU正本との突き合わせそのものはblitz_appが行い、この入口はその報告の行を読むだけである。
//! 正本(`crates/blitz_engine/src/auto_exposure/`)を読めるのはコンポジションルートだけであり、
//! 外部依存を持たないこの入口が正本の写しを持つと、その写しの正しさが検収の前提になってしまう。
//!
//! 決定性を同じ条件の2実行で見られるのは、フレーム数の決まった実行の経過秒が固定の刻みだからである
//! (`crates/blitz_app/src/app/time_of_day/exposure_elapsed.rs`)。対話実行は実時間で進むため、系列は実行ごとに変わる。
//!
//! 3つ目の実行では、積和融合の有無でビンが変わる線形RGBを背景へ流す。全画面の絵だけでは、境界のすぐ近くの
//! 画素がたまたま1つも無い実行で融合の有無が絵に出ない。融合が起きればこの探り色のビンだけが1つずれる。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「順3-IIの実装設計」

mod judgment;
mod parse;
mod run;
mod step_tolerance;
mod summary;

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
    crate::visual_sample_world::用意する().map_err(|破れ| 破れ.to_string())?;
    let 出力先 = PathBuf::from(出力ディレクトリ);
    let 実行環境 = run::実行環境を作る(出力先.clone())?;

    let 一回目 = run::描画して報告を読む(&実行環境, "noon_first", 正午の一日内秒, &run::探り色の扱い::流さない)?;
    let 二回目 = run::描画して報告を読む(&実行環境, "noon_second", 正午の一日内秒, &run::探り色の扱い::流さない)?;
    let 探り = run::描画して報告を読む(&実行環境, "noon_probe", 正午の一日内秒, &run::探り色の扱い::背景へ流す)?;
    judgment::一致を判定する(&一回目)?;
    judgment::二実行の一致を判定する(&一回目, &二回目)?;
    judgment::一致を判定する(&探り)?;
    judgment::探り色を判定する(&探り)?;
    Ok(summary::要約を組む(&一回目, &探り, &出力先))
}
