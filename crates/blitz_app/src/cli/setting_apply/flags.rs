//! 値を伴わない1つのフラグを起動設定へ反映する工程。受け取るのは起動設定と引数の語、返すものは無い。
//! 知らない語を黙って読み飛ばすのは、この工程が値を伴う引数の反映(親モジュール)で処理されなかった語だけを受け取るためである。

use super::super::local_visibility_settings::拡散間接方式の起動上書き;
use super::super::streaming_settings::プレイヤー位置源;
use super::super::temporal_reconstruction_settings::時間再構成方式の起動上書き;
use super::super::{布モード, 描画対象の走査順, 検証計画指定, 粒子表示モード, 起動設定};

pub(super) fn 反映する(設定: &mut 起動設定, 引数値: &str) {
    match 引数値 {
        "--unlit" => 設定.ライティング有効 = false,
        "--particles" => 設定.粒子表示 = 粒子表示モード::粒子トイ,
        "--surface-flow" => 設定.粒子表示 = 粒子表示モード::表面流,
        "--sph-512" => 設定.粒子表示 = 粒子表示モード::Sph512,
        "--sph-1024" => 設定.粒子表示 = 粒子表示モード::Sph1024,
        "--sph-2048" => 設定.粒子表示 = 粒子表示モード::Sph2048,
        "--report-gpu-times" => 設定.gpu時間報告 = true,
        "--report-gpu-frame-times" => 設定.gpu時間のフレーム別生値報告 = true,
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
        "--no-ssao" => 設定.局所可視性.方式の上書き = 拡散間接方式の起動上書き::環境のみで描く,
        "--no-taa" => 設定.時間再構成 = 時間再構成方式の起動上書き::使わないで描く,
        "--report-motion-vector" => 設定.読み戻し検収.動きベクトルを報告するか = true,
        "--report-temporal-reconstruction" => 設定.読み戻し検収.時間再構成を報告するか = true,
        "--dev-ui" => 設定.開発ui初期有効 = true,
        "--debug-cascade-bands" => 設定.画素診断 = blitz_render::cascade::画素診断::距離区分の可視化,
        "--debug-shadow-loss" => 設定.画素診断 = blitz_render::cascade::画素診断::影の欠落計器,
        _ if super::super::time_of_day_flags::反映する(&mut 設定.時間帯, 引数値) => {}
        "--no-post" => 設定.ポスト処理有効 = false,
        "--cloth" => 設定.布モード = 布モード::吊るし布,
        "--cloth-cape" => 設定.布モード = 布モード::マント,
        "--window-rebuild" => 設定.検証計画 = 検証計画指定::ウィンドウ再構築,
        "--shader-reload" => 設定.検証計画 = 検証計画指定::シェーダー差し替え,
        "--streaming" => 設定.ストリーミング.有効 = true,
        "--streaming-route" => 設定.ストリーミング.位置源 = プレイヤー位置源::固定経路,
        "--ow3-dod-route" => 設定.ストリーミング.位置源 = プレイヤー位置源::Ow3Dod経路,
        "--instance-stream-route" => 設定.ストリーミング.位置源 = プレイヤー位置源::インスタンスストリーム経路,
        "--report-streaming" => 設定.ストリーミング.報告する = true,
        "--report-streaming-summary" => 設定.ストリーミング.要約を報告する = true,
        _ => {}
    }
}
