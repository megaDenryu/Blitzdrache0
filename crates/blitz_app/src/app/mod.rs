//! コンポジションルートが所有する `アプリ`(ApplicationHandler実装)。ウィンドウ生成・レンダラー生成・1フレーム実行の配線だけを行い、ロジックは書かない。
mod animation_state;
mod aspect;
mod cloth_frame;
mod cloth_setup;
mod create;
mod draw_dispatch;
mod frame;
mod frame_dump;
mod frame_timing;
mod frame_ui;
mod handler;
mod hot_reload_apply;
mod lod_probe;
mod measurement_setup;
mod particle_setup;
mod queries;
mod scene_camera;
mod scene_lighting;
mod scene_load;
mod scene_read_count;
mod sph_setup;
mod streaming;
mod visibility;
mod window_setup;
use std::path::PathBuf;

use blitz_engine::カメラ;
use blitz_render::{クリアカラー, レンダラー};
use winit::window::Window;

pub(crate) use frame_timing::{フレーム時間統計, 集計する};
pub(crate) use streaming::ストリーミング要約;

use crate::cli::{布モード, 描画対象数, 粒子表示モード, 起動モード};
use crate::dev_ui::開発UI;
use crate::error::起動エラー;
use crate::hot_reload::ホットリローダー;
use crate::input::入力状態;

/// 前提: `レンダラー`フィールドは`window`より前に宣言する。Rustは構造体フィールドを宣言順にDropするため、
/// この順序がレンダラー破棄(surface等)をウィンドウ破棄より必ず先に行うことを保証する(レンダラーの生成前提を満たす)。
pub(crate) struct アプリ {
    レンダラー: Option<レンダラー>,
    window: Option<Window>,
    起動モード: 起動モード,
    シェーダー監視パス: PathBuf,
    シーン名: String,
    アセットルート: PathBuf,
    /// `--global-offset`で世界全体へ加える平行移動。カメラ・照明の大域位置と、チャンク座標から導出した描画アンカーの全部に同じ値を足す。
    大域オフセット: blitz_math::大域ワールド位置,
    描画対象数: Option<描画対象数>,
    ホットリローダー: ホットリローダー,
    カメラ: カメラ,
    入力状態: 入力状態,
    現在フレーム: u32,
    クリア色: クリアカラー,
    ライティング: blitz_render::ライティング入力,
    フレーム構成: blitz_render::フレーム構成,
    粒子表示: 粒子表示モード,
    gpu時間報告: bool,
    gpuメモリ報告: bool,
    描画発行報告: bool,
    フレーム間隔計測: Option<frame_timing::フレーム間隔計測>,
    /// `--report-display-timing`指定でtrue。フレーム時間報告とは独立に切り替える(この計測は描画ループを止めるため)。
    実表示時間報告: bool,
    開発ui: Option<開発UI>,
    開発ui初期有効: bool,
    フレームダンプ先: Option<PathBuf>,
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
    /// `--window-rebuild`指定でtrue。ポスト処理を有効に保ったままウィンドウ再構築を踏む検証計画を選ぶ。
    ウィンドウ再構築検証有効: bool,
    /// `--streaming`指定時だけ`Some`。チャンク格子・目録・予算・台帳・読込器はすべてこの1つの中にある。
    ストリーミング: Option<streaming::ストリーミング配線>,
    /// `--report-streaming-summary`指定でtrue。ストリーミングが無効なときも指定の有無を保つため、配線とは別に持つ。
    ストリーミング要約報告: bool,
    /// インスタンス群の可視判定と個体別LOD。束の可視材料・個体別の段の記憶・毎フレームの可視ID列をここが持つ。
    /// ストリーミングを使わない起動時シーンでも要るため、ストリーミング配線の中ではなくアプリが直に持つ。
    可視判定: visibility::可視判定配線,
    /// `--lod-probe-step`指定時だけ`Some`。段の境界をまたぐ往復を決定的に作るためにカメラを前後させる。
    個体詳細段探査: Option<lod_probe::個体詳細段探査>,
    /// ディスクから実行時シーンを読んだ回数。段の選択や可視判定がディスクI/Oを起こさないことをこの数で示す。
    シーン読込計数: scene_read_count::シーン読込計数,
    起動時エラー: Option<起動エラー>,
}
