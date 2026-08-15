//! 固定構図の採取・対照の焼き付け・判定・計画表示の別と、その別ごとの採取条件。
//! プロセス境界の綴りとの対応は`argument_name`が、採取1回ぶんの条件は`capture_condition`が持つ。

mod argument_name;
mod capture_condition;
mod view_contract;

pub(super) use argument_name::引数を読む;
pub(super) use capture_condition::{採取条件, 読むアセットルート};
pub(super) use view_contract::構図を検査する;

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
    散布の対照を採る,
    散布の候補を採る,
    後処理なし散布の対照を採る,
    後処理なし散布の候補を採る,
    散布の対照を焼く,
    計画を表示する,
    判定する,
    影を判定する,
    散布を判定する,
}

impl 実行の別 {
    pub(super) fn 採取条件(self) -> Option<採取条件> {
        let 始める = 採取条件::名前から始める;
        Some(match self {
            Self::対照を採る => 始める("reference"),
            Self::候補を採る => 始める("candidate"),
            Self::Ssaoなし対照を採る => 始める("reference_no_ssao").ssaoを切る(),
            Self::Ssaoなし候補を採る => 始める("candidate_no_ssao").ssaoを切る(),
            Self::遠景影なし候補を採る => 始める("candidate_no_distant_shadow").遠景影を切る(),
            Self::後処理なし対照を採る => 始める("reference_no_post").後処理を切る(),
            Self::後処理なし候補を採る => 始める("candidate_no_post").後処理を切る(),
            Self::影の対照を採る => 始める("shadow_reference").後処理を切る().明示境界を切る(),
            Self::影の候補を採る => 始める("shadow_candidate").後処理を切る(),
            Self::影の対照の可視度を採る => 始める("shadow_reference_visibility")
                .後処理を切る()
                .影可視度の可視化を立てる()
                .明示境界を切る(),
            Self::影の候補の可視度を採る => 始める("shadow_candidate_visibility").後処理を切る().影可視度の可視化を立てる(),
            Self::散布の対照を採る => 始める("scatter_reference").散布を焼かない対照から読む(),
            Self::散布の候補を採る => 始める("scatter_candidate"),
            Self::後処理なし散布の対照を採る => 始める("scatter_reference_no_post").後処理を切る().散布を焼かない対照から読む(),
            Self::後処理なし散布の候補を採る => 始める("scatter_candidate_no_post").後処理を切る(),
            Self::散布の対照を焼く | Self::計画を表示する | Self::判定する | Self::影を判定する | Self::散布を判定する =>
            {
                return None;
            }
        })
    }
}

pub(super) fn 計画を表示する() -> String {
    "scene=terrain_fox_tour game=fox_tour frames=360 streaming-radius=8 camera-height=terrain+1.5m camera-yaw=180 camera-pitch=8.9893 horizon=upper-third time-of-day=43200 taa=off auto-exposure=off depth-prepass=equal color=RGBA8 depth=D32"
        .to_string()
}
