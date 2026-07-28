//! 1つのCLI引数を起動設定へ反映する。

use std::slice::Iter;

use super::streaming_settings::プレイヤー位置源;
use super::{value_args, 布モード, 粒子表示モード, 起動設定};
use crate::error::起動エラー;

pub(super) fn 反映する(設定: &mut 起動設定, 引数値: &str, 残り: &mut Iter<String>) -> Result<(), 起動エラー> {
    match 引数値 {
        "--frames" => 設定.モード = value_args::frames引数を処理する(残り)?,
        "--benchmark-frames" => 設定.モード = value_args::benchmark_frames引数を処理する(残り)?,
        "--shader-source" => 設定.シェーダー監視パス = value_args::shader_source引数を処理する(残り)?,
        "--scene" => 設定.シーン名 = value_args::scene引数を処理する(残り)?,
        "--asset-root" => 設定.アセットルート = value_args::asset_root引数を処理する(残り)?,
        "--object-count" => 設定.描画対象数 = Some(value_args::object_count引数を処理する(残り)?),
        "--dump-frame" => 設定.フレームダンプ先 = Some(value_args::dump_frame引数を処理する(残り)?),
        "--exposure" => 設定.露出 = value_args::exposure引数を処理する(残り)?,
        "--global-offset" => 設定.大域オフセット = value_args::global_offset引数を処理する(残り)?,
        "--blend" => 設定.ブレンド = value_args::blend引数を処理する(残り)?,
        "--streaming-ram-limit" => {
            設定.ストリーミング.上限.ramバイト数 = value_args::ストリーミング上限引数を処理する(残り, 引数値)?;
        }
        "--streaming-vram-limit" => {
            設定.ストリーミング.上限.vramバイト数 = value_args::ストリーミング上限引数を処理する(残り, 引数値)?;
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
        "--report-frame-times" => 設定.フレーム時間報告 = true,
        "--report-display-timing" => 設定.実表示時間報告 = true,
        "--report-memory" => 設定.gpuメモリ報告 = true,
        "--dev-ui" => 設定.開発ui初期有効 = true,
        "--no-post" => 設定.ポスト処理有効 = false,
        "--cloth" => 設定.布モード = 布モード::吊るし布,
        "--cloth-cape" => 設定.布モード = 布モード::マント,
        "--window-rebuild" => 設定.ウィンドウ再構築検証有効 = true,
        "--streaming" => 設定.ストリーミング.有効 = true,
        "--streaming-route" => 設定.ストリーミング.位置源 = プレイヤー位置源::固定経路,
        "--report-streaming" => 設定.ストリーミング.報告する = true,
        "--report-streaming-summary" => 設定.ストリーミング.要約を報告する = true,
        _ => {}
    }
}
