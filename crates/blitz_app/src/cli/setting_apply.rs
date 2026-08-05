//! 値を伴う1つのCLI引数を起動設定へ反映する。値を伴わないフラグの反映は`flags`が持つ。

mod flags;

use std::slice::Iter;

use super::{
    auto_exposure_probe_args, depth_prepass_args, ibl_step_scan_args, indirect_probe_args, instance_lod_args, lod_crack_args, placement_args,
    screen_pixel_args, shadow_args, time_args, value_args, 起動設定,
};
use crate::error::起動エラー;

pub(super) fn 反映する(設定: &mut 起動設定, 引数値: &str, 残り: &mut Iter<String>) -> Result<(), 起動エラー> {
    match 引数値 {
        "--frames" => 設定.モード = value_args::frames引数を処理する(残り)?,
        "--benchmark-frames" => 設定.モード = value_args::benchmark_frames引数を処理する(残り)?,
        "--ibl-step-scan" | "--ibl-step-control" => 設定.モード = ibl_step_scan_args::引数を処理する(残り, 引数値)?,
        "--shader-source" => 設定.シェーダー監視パス = value_args::shader_source引数を処理する(残り)?,
        "--scene" => 設定.シーン名 = value_args::scene引数を処理する(残り)?,
        "--asset-root" => 設定.アセットルート = value_args::asset_root引数を処理する(残り)?,
        "--object-count" => 設定.描画対象の並べ方.件数 = Some(value_args::object_count引数を処理する(残り)?),
        "--dump-frame" | "--dump-hdr-frame" => value_args::フレームダンプ引数を反映する(&mut 設定.フレームダンプ先, 残り, 引数値)?,
        "--report-sky-pixel" => 設定.読み戻し検収.空の代表画素 = screen_pixel_args::report_sky_pixel引数を処理する(残り)?,
        "--auto-exposure-probe" => 設定.読み戻し検収.自動露出の探り色 = Some(auto_exposure_probe_args::引数を処理する(残り)?),
        "--report-auto-exposure" => 設定.読み戻し検収.自動露出を報告するか = true,
        "--indirect-probe" => 設定.読み戻し検収.遠方環境の検収条件 = Some(indirect_probe_args::引数を処理する(残り)?),
        "--exposure" => 設定.露出 = value_args::exposure引数を処理する(残り)?,
        "--global-offset" => 設定.平行移動.大域ずらし量 = placement_args::global_offset引数を処理する(残り)?,
        "--camera-nudge" => 設定.平行移動.カメラずれ = placement_args::camera_nudge引数を処理する(残り)?,
        "--camera-pitch" => 設定.平行移動.カメラ俯角差分 = placement_args::camera_pitch引数を処理する(残り)?,
        "--camera-yaw" => 設定.平行移動.カメラ方位差分 = placement_args::camera_yaw引数を処理する(残り)?,
        "--shadow-resolution" => 設定.シャドウ計測.一辺解像度 = shadow_args::shadow_resolution引数を処理する(残り)?,
        "--caster-margin" => 設定.シャドウ計測.キャスター余白 = Some(shadow_args::caster_margin引数を処理する(残り)?),
        "--max-shadow-distance" => 設定.シャドウ計測.最大影距離 = Some(shadow_args::max_shadow_distance引数を処理する(残り)?),
        "--shadow-caster-range" => 設定.シャドウ計測.影の視距離 = Some(shadow_args::shadow_caster_range引数を処理する(残り)?),
        "--time-of-day" => 設定.時間帯.一日内時刻の秒 = Some(time_args::time_of_day引数を処理する(残り)?),
        "--time-scale" => 設定.時間帯.時間倍率 = Some(time_args::time_scale引数を処理する(残り)?),
        "--lod-crack-pair" => 設定.ストリーミング.lod継ぎ目検査 = Some(lod_crack_args::引数を処理する(残り)?),
        "--lod-crack-missing" => {
            let 欠落座標 = lod_crack_args::欠落引数を処理する(残り)?;
            let Some(検査) = &mut 設定.ストリーミング.lod継ぎ目検査 else {
                return Err(super::起動引数エラー::Lod継ぎ目検査不正("--lod-crack-pairより先に欠落指定がある".to_string()).into());
            };
            検査.欠落座標 = Some(欠落座標);
        }
        "--blend" => 設定.ブレンド = value_args::blend引数を処理する(残り)?,
        "--lod-probe-step" => 設定.個体詳細段探査刻み = Some(instance_lod_args::lod_probe_step引数を処理する(残り)?),
        "--streaming-ram-limit" => {
            設定.ストリーミング.上限.ramバイト数 = value_args::ストリーミング上限引数を処理する(残り, 引数値)?;
        }
        "--streaming-vram-limit" => {
            設定.ストリーミング.上限.vramバイト数 = value_args::ストリーミング上限引数を処理する(残り, 引数値)?;
        }
        "--streaming-preload-radius" => {
            設定.ストリーミング.先読み半径 = value_args::先読み半径引数を処理する(残り)?;
        }
        "--depth-prepass" => 設定.深度プリパス方式 = depth_prepass_args::引数を処理する(残り)?,
        _ => flags::反映する(設定, 引数値),
    }
    Ok(())
}
