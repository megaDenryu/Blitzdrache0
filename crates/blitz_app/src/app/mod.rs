//! コンポジションルートが所有する `アプリ`(ApplicationHandler実装)。ウィンドウ生成・レンダラー生成・1フレーム実行の配線だけを行い、ロジックは書かない。
mod animation_state;
mod aspect;
mod cloth_frame;
mod cloth_setup;
mod create;
mod draw_dispatch;
pub(crate) mod frame;
mod frame_dump;
mod frame_timing;
mod frame_ui;
mod handler;
mod hot_reload_apply;
mod lod_probe;
mod measurement_setup;
mod particle_setup;
mod primitive_draw_item_registry;
mod queries;
mod report_requests;
mod scene_camera;
mod scene_lighting;
pub(crate) mod scene_load;
mod scene_read_count;
mod section_timing;
mod sph_setup;
mod streaming;
pub(crate) mod time_of_day;
mod visibility;
mod window_setup;
use std::path::PathBuf;

use crate::cli::{布モード, 描画対象の並べ方, 検証計画指定, 空中遠近合成指定, 粒子表示モード, 起動モード};
use crate::dev_ui::開発UI;
use crate::error::起動エラー;
use crate::hot_reload::ホットリローダー;
use crate::input::入力状態;
use blitz_engine::カメラ;
use blitz_render::{クリアカラー, レンダラー};
pub(crate) use frame_timing::{フレーム時間統計, 集計する};
pub(crate) use streaming::ストリーミング要約;
pub(crate) use time_of_day::{太陽天頂区間の記録, 空の再現条件, 遠方環境の鍵の記録, 遠方環境更新判定};
use winit::window::Window;

/// 前提: `レンダラー`フィールドは`window`より前に宣言する。Rustは構造体フィールドを宣言順にDropするため、この順序がレンダラー破棄(surface等)をウィンドウ破棄より必ず先に行うことを保証する(レンダラーの生成前提を満たす)。
pub(crate) struct アプリ {
    レンダラー: Option<レンダラー>,
    window: Option<Window>,
    起動モード: 起動モード,
    シェーダー監視パス: PathBuf,
    シーン名: String,
    アセットルート: PathBuf,
    /// `--global-offset`で世界全体へ加える平行移動。カメラ・照明の大域位置と、チャンク座標から導出した描画の基準原点の全部に同じ値を足す。
    大域ずらし量: blitz_math::大域ワールド位置,
    描画対象の並べ方: 描画対象の並べ方,
    ホットリローダー: ホットリローダー,
    カメラ: カメラ,
    入力状態: 入力状態,
    現在フレーム: u32,
    クリア色: クリアカラー,
    /// 世界の空方針・ゲーム時計・シーンの基準ライティング・そのフレームのライティングと空入力を1つが持つ。
    天空: time_of_day::天空配線,
    フレーム構成: blitz_render::フレーム構成,
    /// 空の放射輝度の評価方式。空パスのシェーダーの選択がこの値で決まり、実行中は変わらない。
    空中遠近合成: 空中遠近合成指定,
    粒子表示: 粒子表示モード,
    /// 終了時に出す報告の要求。起動指定から写した真偽値だけを持つ。
    報告要求: report_requests::報告要求,
    フレーム間隔計測: Option<frame_timing::フレーム間隔計測>,
    開発ui: Option<開発UI>,
    開発ui初期有効: bool,
    計測つまみ: frame::描画の計測つまみ,
    フレームダンプ先: crate::cli::フレームダンプ指定,
    読み戻し検収: crate::cli::読み戻し検収起動設定,
    /// 露出(判断39)とブレンド(判断45)はCLI初期値を開発用UIのスライダーが実行中に書き換える。
    露出: f32,
    ブレンド: f32,
    アニメーション: Option<animation_state::アニメーション再生>,
    布モード: 布モード,
    布プリセット: Option<cloth_setup::布プリセット>,
    /// 掴み操作のエッジ検出用(離した最初のフレームで「離す」介入を発行する)。
    掴み中だった: bool,
    /// アニメーション時刻(秒、毎フレーム1/60秒の固定歩進=判断47)。
    アニメ時刻秒: f32,
    スモーク基準画像: Option<blitz_render::読み戻し画像>,
    /// 起動指定が直に選ぶ検証計画。シーン名から選ぶ既定の計画より先に効き、ピクセル判定を1つも持たない枝を選ぶ。
    検証計画: 検証計画指定,
    /// `--streaming`指定時だけ`Some`。チャンク格子・目録・予算・台帳・読込器はすべてこの1つの中にある。
    ストリーミング: Option<streaming::ストリーミング配線>,
    /// インスタンス群の可視判定と個体別LOD。束の可視材料・個体別の段の記憶・毎フレームの可視ID列をここが持つ。ストリーミングを使わない起動時シーンでも要るため、ストリーミング配線の中ではなくアプリが直に持つ。
    可視判定: visibility::可視判定配線,
    /// 束ごとのプリミティブ描画項目と、それをレンダラーが読む形へ詰め直す受け皿。可視判定の台帳と同じ束IDで対になり、束の追加と解除で一緒に出入りする。
    プリミティブ描画項目台帳: primitive_draw_item_registry::プリミティブ描画項目台帳,
    /// `--report-instance-sections`指定時だけ`Some`。可視判定と個体別LODの1フレーム分の走査が占めた時間を貯める。
    可視個体の選別の計測: Option<section_timing::区間計測>,
    /// `--lod-probe-step`指定時だけ`Some`。段の境界をまたぐ往復を決定的に作るためにカメラを前後させる。
    個体詳細段探査: Option<lod_probe::個体詳細段探査>,
    /// ディスクから実行時シーンを読んだ回数。段の選択や可視判定がディスクI/Oを起こさないことをこの数で示す。
    シーン読込計数: scene_read_count::シーン読込計数,
    起動時エラー: Option<起動エラー>,
}
