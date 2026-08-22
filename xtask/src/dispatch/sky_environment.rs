//! 空・環境系コマンドの割り当て。command_catalogの`sky_environment`分類と同じ範囲(空の描画・
//! 大気のベイク済み画像・遠方環境画像・間接照明の解析値突き合わせなど11件)を担当する。

use std::process::ExitCode;

use crate::{
    atmosphere_lut, derived_environment, distant_environment, indirect_probe, local_visibility, motion_vector, sky_draw, sky_lut, sky_state,
    sky_time, temporal_reconstruction,
};

pub(super) fn 空環境コマンドを割り当てる(名前: &str, 引数一覧: &[String]) -> Option<ExitCode> {
    match 名前 {
        "sky-draw" => Some(sky_draw::空の描画を確認する()),
        "sky-state" => Some(sky_state::天空状態の導出を確認する()),
        "sky-lut" => Some(sky_lut::大気のベイク済み画像の更新を確認する(引数一覧)),
        "atmosphere-lut" => Some(atmosphere_lut::大気のベイク済み画像の生成を確認する()),
        "distant-environment" => Some(distant_environment::遠方環境画像の生成を確認する()),
        "derived-environment" => Some(derived_environment::派生環境画像の生成を確認する()),
        "indirect-probe" => Some(indirect_probe::間接照明の解析値を突き合わせる()),
        "local-visibility" => Some(local_visibility::局所可視度を突き合わせる()),
        "motion-vector" => Some(motion_vector::動きベクトルを確認する()),
        "temporal-reconstruction" => Some(temporal_reconstruction::時間再構成を確認する()),
        "sky-time" => Some(sky_time::時刻別の空を確認する()),
        _ => None,
    }
}
