//! 値を伴う1つのCLI引数を起動設定へ反映する。値を伴わないフラグの反映は`flags`が持つ。

mod flags;

use std::slice::Iter;

use super::{
    auto_exposure_probe_args, cloth_reference_args, depth_prepass_args, game_selection, ibl_step_scan_args, indirect_probe_args, instance_lod_args,
    local_light_count_args, local_visibility_settings, lod_crack_args, path_args, placement_args, point_light_shadow_count_args, screen_pixel_args,
    shadow_args, streaming_value_args, time_args, value_args, 参照比較の床の下の固定点, 起動設定,
};
use crate::error::起動エラー;

pub(super) fn 反映する(設定: &mut 起動設定, 引数値: &str, 残り: &mut Iter<String>) -> Result<(), 起動エラー> {
    match 引数値 {
        "--frames" => 設定.モード = value_args::frames引数を処理する(残り)?,
        "--benchmark-frames" => 設定.モード = value_args::benchmark_frames引数を処理する(残り)?,
        "--ibl-step-scan" | "--ibl-step-control" => 設定.モード = ibl_step_scan_args::引数を処理する(残り, 引数値)?,
        "--shader-source" => 設定.シェーダーの入口ファイル = path_args::shader_source引数を処理する(残り)?,
        "--scene" => 設定.シーン = value_args::scene引数を処理する(残り)?,
        "--game" => 設定.遊ぶゲーム = game_selection::遊ぶゲームの指定の引数を処理する(残り)?,
        "--asset-root" => 設定.アセットの置き場 = path_args::asset_root引数を処理する(残り)?,
        "--object-count" => 設定.描画対象の並べ方.件数 = Some(value_args::object_count引数を処理する(残り)?),
        "--dump-frame" | "--dump-hdr-frame" | "--dump-depth-frame" => {
            value_args::フレームダンプ引数を反映する(&mut 設定.フレームダンプ先, 残り, 引数値)?
        }
        "--report-sky-pixel" => 設定.読み戻し検収.空の代表画素 = screen_pixel_args::report_sky_pixel引数を処理する(残り)?,
        "--auto-exposure-probe" => 設定.読み戻し検収.自動露出の探り色 = Some(auto_exposure_probe_args::引数を処理する(残り)?),
        "--report-auto-exposure" => 設定.読み戻し検収.自動露出を報告するか = true,
        "--indirect-probe" => 設定.読み戻し検収.遠方環境の検収条件 = Some(indirect_probe_args::引数を処理する(残り)?),
        "--exposure" => 設定.露出 = value_args::exposure引数を処理する(残り)?,
        "--cloth-xpbd-reference" => {
            設定.布モード = cloth_reference_args::cloth_xpbd_reference引数を処理する(残り, 引数値, 参照比較の床の下の固定点::持たない)?
        }
        "--cloth-xpbd-reference-below-floor" => {
            設定.布モード = cloth_reference_args::cloth_xpbd_reference引数を処理する(残り, 引数値, 参照比較の床の下の固定点::持つ)?
        }
        "--cloth-xpbd-reference-bending" => {
            設定.布モード = cloth_reference_args::cloth_xpbd_reference_bending引数を処理する(残り, 設定.布モード)?
        }
        "--cloth-xpbd-reference-shape" => 設定.布モード = cloth_reference_args::cloth_xpbd_reference_shape引数を処理する(残り, 設定.布モード)?,
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
        "--local-light-count" => 設定.ライティング.局所光の件数 = local_light_count_args::引数を処理する(残り)?,
        "--point-light-shadow-count" => 設定.ライティング.影を落とす灯の件数 = point_light_shadow_count_args::引数を処理する(残り)?,
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
        "--streaming-route-start-east-meters"
        | "--streaming-route-start-south-meters"
        | "--streaming-route-end-east-meters"
        | "--streaming-route-end-south-meters"
        | "--streaming-route-meters-per-frame" => {
            streaming_value_args::固定経路引数を反映する(&mut 設定.ストリーミング.固定経路, 残り, 引数値)?;
        }
        "--streaming-loader-workers" | "--streaming-request-capacity" | "--streaming-completion-capacity" => {
            設定.ストリーミング.読込 = streaming_value_args::読込引数を反映する(設定.ストリーミング.読込, 残り, 引数値)?;
        }
        "--depth-prepass" => 設定.深度プリパス方式 = Some(depth_prepass_args::引数を処理する(残り)?),
        "--local-visibility-shape" => 設定.読み戻し検収.局所可視性の検収の形 = Some(local_visibility_settings::形の引数を処理する(残り)?),
        "--local-visibility-fixed" => 設定.局所可視性.可視度の固定 = local_visibility_settings::固定の引数を処理する(残り)?,
        _ => flags::反映する(設定, 引数値),
    }
    Ok(())
}
