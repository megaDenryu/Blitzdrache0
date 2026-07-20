//! コンポジションルートが所有する `アプリ`(ApplicationHandler実装)。
//! ウィンドウ生成・レンダラー生成・1フレーム実行の配線だけを行い、ロジックは書かない。

mod animation_state;
mod aspect;
mod draw_dispatch;
mod frame;
mod frame_ui;
mod frame_dump;
mod handler;
mod queries;
mod hot_reload_apply;
mod scene_camera;
mod scene_load;
mod window_setup;

use std::path::PathBuf;

use blitz_engine::カメラ;
use blitz_render::{クリアカラー, レンダラー};
use winit::window::Window;

use scene_camera::シーン初期カメラを作る;

use crate::cli::{起動モード, 起動設定};
use crate::dev_ui::開発UI;
use crate::error::起動エラー;
use crate::hot_reload::ホットリローダー;
use crate::input::入力状態;

/// 前提: `レンダラー` フィールドは `window` より前に宣言する。Rustは構造体フィールドを
/// 宣言順にDropするため、この順序がレンダラー破棄(surface等の破棄)を
/// ウィンドウ破棄より必ず先に行うことを保証する(レンダラーの生成前提を満たす)。
pub(crate) struct アプリ {
    レンダラー: Option<レンダラー>,
    window: Option<Window>,
    起動モード: 起動モード,
    シェーダー監視パス: PathBuf,
    シーン名: String,
    アセットルート: PathBuf,
    ホットリローダー: ホットリローダー,
    カメラ: カメラ,
    入力状態: 入力状態,
    現在フレーム: u32,
    クリア色: クリアカラー,
    ライティング有効: bool,
    粒子有効: bool,
    gpu時間報告: bool,
    /// resumed時にウィンドウ生成後に構築する(判断34)。それまでは`None`。
    開発ui: Option<開発UI>,
    開発ui初期有効: bool,
    フレームダンプ先: Option<PathBuf>,
    ポスト処理有効: bool,
    /// トーンマップ前にHDR輝度へ掛ける露出倍率(判断39)。CLI初期値を開発用UIのスライダーが実行中に書き換える。
    露出: f32,
    /// アニメーションクリップ2本のブレンド係数(判断45)。CLI初期値を開発用UIのスライダーが実行中に書き換える。
    ブレンド: f32,
    /// スキン付き+クリップ有りシーンのみ`Some`。resumed時のシーン読込で構築する。
    アニメーション: Option<animation_state::アニメーション再生>,
    /// アニメーション時刻(秒)。毎フレーム1/60秒の固定歩進(判断47)。
    アニメ時刻秒: f32,
    /// foxスモークの差分判定用の基準画像(fox基準保存アクションで保存する)。
    スモーク基準画像: Option<blitz_render::読み戻し画像>,
    起動時エラー: Option<起動エラー>,
}

impl アプリ {
    pub(crate) fn 生成する(起動設定: 起動設定, クリア色: クリアカラー) -> Self {
        Self {
            レンダラー: None,
            window: None,
            起動モード: 起動設定.モード,
            ホットリローダー: ホットリローダー::生成する(起動設定.シェーダー監視パス.clone()),
            シェーダー監視パス: 起動設定.シェーダー監視パス,
            カメラ: シーン初期カメラを作る(&起動設定.シーン名),
            シーン名: 起動設定.シーン名,
            アセットルート: 起動設定.アセットルート,
            入力状態: 入力状態::生成する(),
            現在フレーム: 0,
            クリア色,
            ライティング有効: 起動設定.ライティング有効,
            粒子有効: 起動設定.粒子有効,
            gpu時間報告: 起動設定.gpu時間報告,
            開発ui: None,
            開発ui初期有効: 起動設定.開発ui初期有効,
            フレームダンプ先: 起動設定.フレームダンプ先,
            ポスト処理有効: 起動設定.ポスト処理有効,
            露出: 起動設定.露出,
            ブレンド: 起動設定.ブレンド,
            アニメーション: None,
            アニメ時刻秒: 0.0,
            スモーク基準画像: None,
            起動時エラー: None,
        }
    }

}
