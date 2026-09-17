//! コンポジションルートが所有する `アプリ`(ApplicationHandler実装)。ウィンドウ生成・レンダラー生成・1フレーム実行の配線だけを行い、ロジックは書かない。
mod animation_state;
mod aspect;
pub(crate) mod cloth_reference;
mod cloth_setup;
mod cloth_wiring;
mod create;
mod draw_bundle_ledger;
mod draw_dispatch;
mod exit_report;
pub(crate) mod frame;
mod frame_dump;
mod frame_timing;
mod frame_ui;
mod handler;
mod one_time_launch_settings;
mod particle_setup;
mod persistent_bundles;
mod primitive_draw_item_registry;
mod queries;
mod report_requests;
mod resource_wiring;
mod scene_camera;
mod scene_lighting;
pub(crate) mod scene_load;
mod screen_installation;
mod self_operation_wiring;
mod sph_setup;
mod streaming;
pub(crate) mod time_of_day;
mod time_step;
mod verification_launch_settings;
mod verification_observation;
mod visibility;
mod window_setup;
use crate::cli::{描画対象の並べ方, 起動モード};
use crate::{error::起動エラー, input::入力状態};
use blitz_render::クリアカラー;
use cloth_wiring::布の配線;
use draw_bundle_ledger::描画束の台帳;
pub(crate) use frame_timing::{フレーム時間統計, フレーム間隔から統計を集計する};
use one_time_launch_settings::起動時に一度だけ使う設定;
use resource_wiring::資源の配線;
use screen_installation::画面の据え付け;
use self_operation_wiring::自己操作の配線;
pub(crate) use time_of_day::{太陽天頂区間の記録, 空の再現条件, 遠方環境の鍵の記録, 遠方環境更新判定};
pub(crate) use time_step::{フレーム番号, 描画補間の割合, 進める刻み数};
use verification_launch_settings::検収の起動設定;
use verification_observation::検収の観測;
pub(crate) use {draw_dispatch::時間再構成の突き合わせの要約, streaming::ストリーミング要約};

/// `据え付け`は、レンダラー・画面へ重ねるUI・ウィンドウが同じ地点で揃って据わり揃って消えることを1つで持つ。破棄順の不変条件はその型が持つ。
///
/// `大域ずらし量`は、カメラ・照明の大域位置と、チャンク座標から導出した描画の基準原点の全部に同じ値を足す。
/// `天空`は世界の空方針・ゲーム時計・シーンの基準ライティング・そのフレームのライティングと空入力を1つで持つ。
/// `露出`(判断39)と`ブレンド`(判断45)は、CLIの初期値を開発用UIのスライダーが実行中に書き換える。
/// `時間進行`は基本刻みと一描画で進める刻み数の上限、実行の種類で選んだ進め方、および今から描く描画機会のフレーム番号を1つで持つ。刻みと描画機会を数える状態だけを束ね、固定刻みで確定するゲーム状態も描画機会ごとの一時状態も混ぜない。
/// `資源の配線`は、ホットリローダーとチャンクのストリーミングを1つで持つ。ストリーミングの中に、チャンク格子・目録・予算・台帳・読込器はすべて入っている。
/// `描画束の台帳`は、束ごとの可視材料・束ごとのプリミティブ描画項目・常駐する束の状態・世界に1つ置く地表の層のタイルを1つで持つ。束の登録と解除で4つが一体に動く。
/// `検収の観測`は、フレーム間隔の計測・可視個体の選別の計測・シーン読込の計数・時間再構成の前フレームの結果・スモークの基準画像という、実行を通して積み上がる観測を1つで持つ。
/// `検収の起動設定`は、読み戻し検収の指定・フレームダンプ先・終了時の報告要求を1つで持ち、この描画の検収の経路を判定する。
/// `自己操作`は、スモーク実行と個体詳細段探査を1つで持ち、アプリが自分でウィンドウとカメラを動かして決定的な絵を作る。
pub(crate) struct アプリ {
    据え付け: Option<画面の据え付け>,
    起動モード: 起動モード,
    一度だけ使う設定: Option<起動時に一度だけ使う設定>, // 起動の途中で`resume`が消費し、以後は`None`
    大域ずらし量: blitz_math::大域ワールド位置,         // `--global-offset`で世界全体へ加える平行移動
    描画対象の並べ方: 描画対象の並べ方,
    カメラ: blitz_engine::カメラ,
    入力状態: 入力状態,
    ゲーム配線: crate::game::ゲーム配線,
    時間進行: time_step::時間進行配線, // その描画で固定刻みを何本進めるかと、この描画のフレーム番号を決める配線
    視点の履歴: frame::視点の履歴,
    クリア色: クリアカラー,
    天空: time_of_day::天空配線, // 空と時刻の配線
    世界の描画構成: create::世界の描画構成,
    検収の起動設定: 検収の起動設定,
    検収の観測: 検収の観測,
    計測つまみ: frame::描画の計測つまみ,
    露出: crate::cli::露出倍率,
    ブレンド: crate::cli::アニメーションのブレンド係数,
    アニメーション: Option<animation_state::アニメーション再生>,
    布の配線: 布の配線,
    アニメ時刻: blitz_math::秒, // アニメーション時刻(その描画で進めた刻み数×基本刻みで歩進する)
    資源の配線: 資源の配線,     // 外から来た新しいデータを描画へ載せる2つ(ホットリローダー・ストリーミング)
    描画束の台帳: 描画束の台帳, // 束の登録と解除で一体に動く4つ(可視材料・プリミティブ描画項目・常駐束・地表の層のタイル)
    自己操作: 自己操作の配線,
    起動時エラー: Option<起動エラー>,
}
