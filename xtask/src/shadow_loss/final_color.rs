//! 本番の見た目の2枚の撮影。受け取るのは指定、返すのは2枚のパスを述べた要約である。
//!
//! 数える様式と違い、診断出力を付けず、空とポスト処理も外さない。オーナーが2枚を並べて見た目で選ぶための絵だからである。
//! **比較も判定もしない。** 最終色は材質・PCF・露出・ポスト処理を混ぜるため定量に使わないことが確定しており、
//! ここで画素を数えるとその契約を裏口から破ることになる。判断は絵を見る人が下す。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「シャドウ性能の是正(フェーズ2性能課題、2026-08-03着手)」

use std::path::{Path, PathBuf};
use std::process::Command;

use super::args::指定;
use super::candidate_axis::候補の計測指定;
use super::scene_choice::構図;

pub(super) fn 二枚を撮る(指定: &指定) -> Result<String, String> {
    let 出力先 = super::描く支度をする(指定.構図)?;
    let 基準 = 撮る(&出力先, &出力名を作る(&指定.候補, "baseline"), 指定.構図, &[])?;
    let 候補 = 撮る(&出力先, &出力名を作る(&指定.候補, "candidate"), 指定.構図, &指定.候補.起動指定へ写す())?;
    Ok(format!(
        "最終色の2枚を撮った(比較も判定もしない)。基準は{}、候補は{}",
        基準.display(),
        候補.display()
    ))
}

/// 撮った絵の名前。軸と距離を名前へ入れるのは、αの組とβの組を続けて撮ったときに上書きし合わないためである。
/// 4枚を並べて選ぶには4枚が同時にディスクへ残っている必要がある。
fn 出力名を作る(候補: &候補の計測指定, 役: &str) -> String {
    let 軸 = 候補.綴り().trim_start_matches("--").replace('-', "_");
    format!("final_{軸}_{}_{役}", 候補.距離の綴り())
}

fn 撮る(出力先: &Path, 出力名: &str, 構図: 構図, 候補の起動指定: &[String]) -> Result<PathBuf, String> {
    let ダンプ先 = PathBuf::from(出力先).join(出力名);
    let mut コマンド = Command::new("cargo");
    コマンド
        .args(["run", "-p", "blitz_app", "--", "--scene", 構図.シーン名()])
        .args(["--asset-root", 構図.アセットルート()])
        .args(["--frames", super::フレーム数])
        .args(["--time-of-day", super::一日内秒])
        .arg("--no-taa")
        .args(構図.追加の起動指定())
        .args(候補の起動指定)
        .arg("--dump-frame")
        .arg(&ダンプ先);
    let 出力 = コマンド
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({出力名}): {誤り}"))?;
    if !出力.status.success() {
        return Err(format!("blitz_appが{}で失敗した({出力名})", 出力.status));
    }
    crate::validation_count::零件数を確かめる(&String::from_utf8_lossy(&出力.stdout), 出力名)?;
    crate::raw_png::変換する(&ダンプ先)
}
