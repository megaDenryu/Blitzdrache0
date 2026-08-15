//! 引数を固定構図の対照採取・候補採取・計画表示へ写す。

mod view_contract;

pub(super) use view_contract::構図を検査する;

use super::error::遠景構図の検収エラー;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 実行の別 {
    対照を採る,
    候補を採る,
    Ssaoなし対照を採る,
    Ssaoなし候補を採る,
    遠景影なし候補を採る,
    後処理なし対照を採る,
    後処理なし候補を採る,
    影の対照を採る,
    影の候補を採る,
    影の対照の可視度を採る,
    影の候補の可視度を採る,
    計画を表示する,
    判定する,
    影を判定する,
}

pub(super) struct 採取条件 {
    pub(super) 名前: &'static str,
    pub(super) ssaoを使わない: bool,
    pub(super) 遠景影を使わない: bool,
    /// 光のにじみと明るさの圧縮を組まない構成で採る指定。主判定はこの対で近傍不変を課す。
    /// 参照: `_doc/設計/大規模世界の生成と遠景.md`第5段階の検査点4
    pub(super) 後処理を使わない: bool,
    /// 本番の色の代わりに影の欠落計器の診断色を出す指定。影の検査点が色差の帰属に使う。
    /// 参照: `_doc/設計/大規模世界の生成と遠景.md`第5段階の検査点5
    pub(super) 影可視度を可視化する: bool,
}

impl 実行の別 {
    pub(super) fn 採取条件(self) -> Option<採取条件> {
        let (名前, ssaoを使わない, 遠景影を使わない, 後処理を使わない, 影可視度を可視化する) = match self {
            Self::対照を採る => ("reference", false, false, false, false),
            Self::候補を採る => ("candidate", false, false, false, false),
            Self::Ssaoなし対照を採る => ("reference_no_ssao", true, false, false, false),
            Self::Ssaoなし候補を採る => ("candidate_no_ssao", true, false, false, false),
            Self::遠景影なし候補を採る => ("candidate_no_distant_shadow", false, true, false, false),
            Self::後処理なし対照を採る => ("reference_no_post", false, false, true, false),
            Self::後処理なし候補を採る => ("candidate_no_post", false, false, true, false),
            Self::影の対照を採る => ("shadow_reference", false, false, true, false),
            Self::影の候補を採る => ("shadow_candidate", false, false, true, false),
            Self::影の対照の可視度を採る => ("shadow_reference_visibility", false, false, true, true),
            Self::影の候補の可視度を採る => ("shadow_candidate_visibility", false, false, true, true),
            Self::計画を表示する | Self::判定する | Self::影を判定する => return None,
        };
        Some(採取条件 {
            名前,
            ssaoを使わない,
            遠景影を使わない,
            後処理を使わない,
            影可視度を可視化する,
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
        [引数] if 引数 == "--capture-shadow-reference" => Ok(実行の別::影の対照を採る),
        [引数] if 引数 == "--capture-shadow-candidate" => Ok(実行の別::影の候補を採る),
        [引数] if 引数 == "--capture-shadow-reference-visibility" => Ok(実行の別::影の対照の可視度を採る),
        [引数] if 引数 == "--capture-shadow-candidate-visibility" => Ok(実行の別::影の候補の可視度を採る),
        [引数] if 引数 == "--judge" => Ok(実行の別::判定する),
        [引数] if 引数 == "--judge-shadow" => Ok(実行の別::影を判定する),
        _ => Err(遠景構図の検収エラー::引数が不正(引数一覧.join(" "))),
    }
}

pub(super) fn 計画を表示する() -> String {
    "scene=terrain_fox_tour game=fox_tour frames=360 streaming-radius=8 camera-height=terrain+1.5m camera-yaw=180 camera-pitch=8.9893 horizon=upper-third time-of-day=43200 taa=off auto-exposure=off depth-prepass=equal color=RGBA8 depth=D32"
        .to_string()
}
