//! 空・環境系コマンドの割り当て。command_catalogの`sky_environment`分類と同じ範囲(空の描画・
//! 大気のベイク済み画像・遠方環境画像・間接照明の解析値突き合わせなど11件)を担当する。

use std::process::ExitCode;

use crate::{
    atmosphere_lut, derived_environment, distant_environment, indirect_probe, local_visibility, motion_vector, sky_draw, sky_lut, sky_state,
    sky_time, temporal_reconstruction,
};

pub(super) fn 割り当てる(名前: &str, 引数一覧: &[String]) -> Option<ExitCode> {
    match 名前 {
        "sky-draw" => Some(sky_draw::実行する()),
        "sky-state" => Some(sky_state::実行する()),
        "sky-lut" => Some(sky_lut::実行する(引数一覧)),
        "atmosphere-lut" => Some(atmosphere_lut::実行する()),
        "distant-environment" => Some(distant_environment::実行する()),
        "derived-environment" => Some(derived_environment::実行する()),
        "indirect-probe" => Some(indirect_probe::実行する()),
        "local-visibility" => Some(local_visibility::実行する()),
        "motion-vector" => Some(motion_vector::実行する()),
        "temporal-reconstruction" => Some(temporal_reconstruction::実行する()),
        "sky-time" => Some(sky_time::実行する()),
        _ => None,
    }
}
