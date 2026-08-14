//! 引数を固定構図の対照採取・候補採取・計画表示へ写す。

use super::error::遠景構図の検収エラー;
use crate::acceptance::終了時報告;
use crate::report_heading::報告の見出し;

const プレイヤー位置: 報告の見出し = 報告の見出し::定数から生成する("プレイヤーの大域位置:");
const カメラ位置: 報告の見出し = 報告の見出し::定数から生成する("カメラの大域位置:");
const 視点高メートル: f64 = 1.5;
const 丸め許容メートル: f64 = 0.000_002;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 実行の別 {
    対照を採る,
    候補を採る,
    計画を表示する,
    判定する,
}

pub(super) fn 引数を読む(引数一覧: &[String]) -> Result<実行の別, 遠景構図の検収エラー> {
    match 引数一覧 {
        [引数] if 引数 == "--capture-reference" => Ok(実行の別::対照を採る),
        [引数] if 引数 == "--capture-candidate" => Ok(実行の別::候補を採る),
        [引数] if 引数 == "--print-plan" => Ok(実行の別::計画を表示する),
        [引数] if 引数 == "--judge" => Ok(実行の別::判定する),
        _ => Err(遠景構図の検収エラー::引数が不正(引数一覧.join(" "))),
    }
}

pub(super) fn 計画を表示する() -> String {
    "scene=terrain_fox_tour game=fox_tour frames=360 streaming-radius=8 camera-height=terrain+1.5m camera-yaw=180 camera-pitch=8.9893 horizon=upper-third time-of-day=43200 taa=off auto-exposure=off depth-prepass=equal color=RGBA8 depth=D32"
        .to_string()
}

pub(super) fn 構図を検査する(報告: &終了時報告) -> Result<String, String> {
    let 地表高 = 天頂成分を読む(報告.見出しに続く本文(&プレイヤー位置).map_err(|誤り| 誤り.to_string())?)?;
    let カメラ高 = 天頂成分を読む(報告.見出しに続く本文(&カメラ位置).map_err(|誤り| 誤り.to_string())?)?;
    let 差 = カメラ高 - 地表高;
    if (差 - 視点高メートル).abs() > 丸め許容メートル {
        return Err(format!("カメラ高{カメラ高:.6}mが地表高{地表高:.6}m+{視点高メートル:.1}mではない"));
    }
    Ok(format!("地表高{地表高:.6}m・カメラ高{カメラ高:.6}m・視点高{差:.6}m"))
}

fn 天頂成分を読む(本文: &str) -> Result<f64, String> {
    本文
        .split_whitespace()
        .find_map(|語| 語.strip_prefix("天頂"))
        .ok_or_else(|| format!("大域位置に天頂成分が無い: {本文}"))?
        .parse::<f64>()
        .map_err(|誤り| format!("大域位置の天頂成分を読めない: {誤り}"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{実行の別, 引数を読む, 構図を検査する};
    use crate::acceptance::{検収の実行名, 終了時報告};

    #[test]
    fn 三つの実行の別を一つずつ受け付ける() {
        for (綴り, 期待) in [
            ("--capture-reference", 実行の別::対照を採る),
            ("--capture-candidate", 実行の別::候補を採る),
            ("--print-plan", 実行の別::計画を表示する),
            ("--judge", 実行の別::判定する),
        ] {
            assert_eq!(引数を読む(&[綴り.to_string()]).unwrap(), 期待);
        }
    }

    #[test]
    fn 複数の実行指定を拒む() {
        let 引数 = ["--capture-reference".to_string(), "--capture-candidate".to_string()];
        assert!(引数を読む(&引数).is_err());
    }

    #[test]
    fn 報告したカメラ高が地表高より一メートル半高いことを課す() {
        let 正常 = 報告("3.750000");
        let 低すぎる = 報告("3.749000");
        assert!(構図を検査する(&正常).is_ok());
        assert!(構図を検査する(&低すぎる).is_err());
    }

    fn 報告(カメラ高: &str) -> 終了時報告 {
        let 本文 = format!("  プレイヤーの大域位置: 東0.000000 天頂2.250000 南0.000000\n  カメラの大域位置: 東0.000000 天頂{カメラ高} 南9.000000\n");
        終了時報告::取り込む(&検収の実行名::生成する("view_contract").unwrap(), 本文, String::new())
    }
}
