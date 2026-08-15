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
    Ssaoなし対照を採る,
    Ssaoなし候補を採る,
    遠景影なし候補を採る,
    後処理なし対照を採る,
    後処理なし候補を採る,
    計画を表示する,
    判定する,
}

pub(super) struct 採取条件 {
    pub(super) 名前: &'static str,
    pub(super) ssaoを使わない: bool,
    pub(super) 遠景影を使わない: bool,
    /// 光のにじみと明るさの圧縮を組まない構成で採る指定。主判定はこの対で近傍不変を課す。
    /// 参照: `_doc/設計/大規模世界の生成と遠景.md`第5段階の検査点4
    pub(super) 後処理を使わない: bool,
}

impl 実行の別 {
    pub(super) fn 採取条件(self) -> Option<採取条件> {
        let (名前, ssaoを使わない, 遠景影を使わない, 後処理を使わない) = match self {
            Self::対照を採る => ("reference", false, false, false),
            Self::候補を採る => ("candidate", false, false, false),
            Self::Ssaoなし対照を採る => ("reference_no_ssao", true, false, false),
            Self::Ssaoなし候補を採る => ("candidate_no_ssao", true, false, false),
            Self::遠景影なし候補を採る => ("candidate_no_distant_shadow", false, true, false),
            Self::後処理なし対照を採る => ("reference_no_post", false, false, true),
            Self::後処理なし候補を採る => ("candidate_no_post", false, false, true),
            Self::計画を表示する | Self::判定する => return None,
        };
        Some(採取条件 {
            名前,
            ssaoを使わない,
            遠景影を使わない,
            後処理を使わない,
        })
    }
}

pub(super) fn 引数を読む(引数一覧: &[String]) -> Result<実行の別, 遠景構図の検収エラー> {
    match 引数一覧 {
        [引数] if 引数 == "--capture-reference" => Ok(実行の別::対照を採る),
        [引数] if 引数 == "--capture-candidate" => Ok(実行の別::候補を採る),
        [引数] if 引数 == "--capture-reference-no-ssao" => Ok(実行の別::Ssaoなし対照を採る),
        [引数] if 引数 == "--capture-candidate-no-ssao" => Ok(実行の別::Ssaoなし候補を採る),
        [引数] if 引数 == "--capture-candidate-no-distant-shadow" => Ok(実行の別::遠景影なし候補を採る),
        [引数] if 引数 == "--capture-reference-no-post" => Ok(実行の別::後処理なし対照を採る),
        [引数] if 引数 == "--capture-candidate-no-post" => Ok(実行の別::後処理なし候補を採る),
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
