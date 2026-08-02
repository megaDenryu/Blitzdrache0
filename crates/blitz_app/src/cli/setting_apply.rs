//! 1つのCLI引数を起動設定へ反映する。

use std::slice::Iter;

use super::streaming_settings::プレイヤー位置源;
use super::{
    instance_lod_args, lod_crack_args, placement_args, screen_pixel_args, shadow_args, time_args, value_args, 布モード, 描画対象の走査順,
    粒子表示モード, 起動設定,
};
use crate::error::起動エラー;

pub(super) fn 反映する(設定: &mut 起動設定, 引数値: &str, 残り: &mut Iter<String>) -> Result<(), 起動エラー> {
    match 引数値 {
        "--frames" => 設定.モード = value_args::frames引数を処理する(残り)?,
        "--benchmark-frames" => 設定.モード = value_args::benchmark_frames引数を処理する(残り)?,
        "--shader-source" => 設定.シェーダー監視パス = value_args::shader_source引数を処理する(残り)?,
        "--scene" => 設定.シーン名 = value_args::scene引数を処理する(残り)?,
        "--asset-root" => 設定.アセットルート = value_args::asset_root引数を処理する(残り)?,
        "--object-count" => 設定.描画対象の並べ方.件数 = Some(value_args::object_count引数を処理する(残り)?),
        "--dump-frame" => 設定.フレームダンプ先 = Some(value_args::dump_frame引数を処理する(残り)?),
        "--report-sky-pixel" => 設定.空の代表画素報告 = screen_pixel_args::report_sky_pixel引数を処理する(残り)?,
        "--exposure" => 設定.露出 = value_args::exposure引数を処理する(残り)?,
        "--global-offset" => 設定.平行移動.大域ずらし量 = placement_args::global_offset引数を処理する(残り)?,
        "--camera-nudge" => 設定.平行移動.カメラずれ = placement_args::camera_nudge引数を処理する(残り)?,
        "--camera-pitch" => 設定.平行移動.カメラ俯角差分 = placement_args::camera_pitch引数を処理する(残り)?,
        "--camera-yaw" => 設定.平行移動.カメラ方位差分 = placement_args::camera_yaw引数を処理する(残り)?,
        "--shadow-resolution" => 設定.シャドウ計測.一辺解像度 = shadow_args::shadow_resolution引数を処理する(残り)?,
        "--caster-margin" => 設定.シャドウ計測.キャスター余白 = Some(shadow_args::caster_margin引数を処理する(残り)?),
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
        _ => フラグを反映する(設定, 引数値),
    }
    Ok(())
}

fn フラグを反映する(設定: &mut 起動設定, 引数値: &str) {
    match 引数値 {
        "--unlit" => 設定.ライティング有効 = false,
        "--particles" => 設定.粒子表示 = 粒子表示モード::粒子トイ,
        "--surface-flow" => 設定.粒子表示 = 粒子表示モード::表面流,
        "--sph-512" => 設定.粒子表示 = 粒子表示モード::Sph512,
        "--sph-1024" => 設定.粒子表示 = 粒子表示モード::Sph1024,
        "--sph-2048" => 設定.粒子表示 = 粒子表示モード::Sph2048,
        "--report-gpu-times" => 設定.gpu時間報告 = true,
        "--report-atmosphere-passes" => 設定.大気のベイク済み画像パス数報告 = true,
        "--report-frame-times" => 設定.フレーム時間報告 = true,
        "--report-display-timing" => 設定.実表示時間報告 = true,
        "--report-memory" => 設定.gpuメモリ報告 = true,
        "--report-draw-issue" => 設定.描画発行報告 = true,
        "--report-sun-angle" => 設定.太陽角度報告 = true,
        "--report-caster-distance" => 設定.キャスター距離分布報告 = true,
        "--report-instance-sections" => 設定.インスタンス区間報告 = true,
        "--reverse-draw-order" => 設定.描画対象の並べ方.走査順 = 描画対象の走査順::逆順,
        "--no-instance-cull" => 設定.インスタンス可視判定有効 = false,
        "--no-instance-lod" => 設定.インスタンス段選択有効 = false,
        "--no-instance-shadow" => 設定.インスタンス影キャスター有効 = false,
        "--no-shadow-casters" => 設定.影キャスター全体有効 = false,
        "--dev-ui" => 設定.開発ui初期有効 = true,
        "--debug-cascade-bands" => 設定.距離区分の可視化 = true,
        "--sky" => 設定.時間帯.空 = super::空の起動指定::空ありとして扱う,
        "--no-sky" => 設定.時間帯.空 = super::空の起動指定::空を描かない,
        "--no-aerial-composite" => 設定.時間帯.空中遠近合成 = super::空中遠近合成指定::合成しない,
        "--sun-disk-off" => 設定.時間帯.太陽円盤 = super::太陽円盤指定::消す,
        "--no-post" => 設定.ポスト処理有効 = false,
        "--cloth" => 設定.布モード = 布モード::吊るし布,
        "--cloth-cape" => 設定.布モード = 布モード::マント,
        "--window-rebuild" => 設定.ウィンドウ再構築検証有効 = true,
        "--streaming" => 設定.ストリーミング.有効 = true,
        "--streaming-route" => 設定.ストリーミング.位置源 = プレイヤー位置源::固定経路,
        "--ow3-dod-route" => 設定.ストリーミング.位置源 = プレイヤー位置源::Ow3Dod経路,
        "--instance-stream-route" => 設定.ストリーミング.位置源 = プレイヤー位置源::インスタンスストリーム経路,
        "--report-streaming" => 設定.ストリーミング.報告する = true,
        "--report-streaming-summary" => 設定.ストリーミング.要約を報告する = true,
        _ => {}
    }
}
