//! resumed時のウィンドウ・レンダラー生成。カタログ構築・シーン読込・
//! アセットホットリロード監視の初期設定もここで行う。

use blitz_engine::アセットID;
use blitz_render::{ウィンドウ寸法, レンダラー};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

use super::animation_state::アニメーション再生;
use super::scene_load;
use crate::dev_ui::開発UI;
use crate::embedded_shaders;
use crate::error::起動エラー;
use crate::hot_reload::ホットリローダー;

const 初期幅: u32 = 1280;
const 初期高さ: u32 = 720;

/// ウィンドウを生成し、そのハンドルからレンダラー・開発用UIを生成する。
///
/// 前提: 戻り値のタプルはこの順でアプリ構造体のフィールドへ格納され、
/// windowがレンダラーより先にDropされないことをフィールド宣言順で保証する。
#[allow(clippy::too_many_arguments)]
pub(super) fn ウィンドウとレンダラーを作る(
    event_loop: &ActiveEventLoop,
    シーン名: &str,
    アセットルート: &std::path::Path,
    ホットリローダー: &mut ホットリローダー,
    粒子有効: bool,
    開発ui初期有効: bool,
    ポスト処理有効: bool,
) -> Result<(Window, レンダラー, 開発UI, Option<アニメーション再生>), 起動エラー> {
    let window = event_loop.create_window(
        WindowAttributes::default()
            .with_title("Blitzdrache0")
            .with_inner_size(PhysicalSize::new(初期幅, 初期高さ)),
    )?;

    let 表示ハンドル = window.display_handle()?.as_raw();
    let ウィンドウハンドル = window.window_handle()?.as_raw();
    let 物理寸法 = window.inner_size();
    let 寸法 = ウィンドウ寸法::生成する(物理寸法.width, 物理寸法.height);
    let シェーダー束 = embedded_shaders::埋め込みシェーダー束を生成する(粒子有効)?;

    let カタログ = scene_load::カタログを構築する(アセットルート)?;
    let (シーン, 頂点一覧, インデックス一覧, マテリアル) =
        scene_load::シーンを読み込んで変換する(&カタログ, シーン名)?;
    let スキン素材 = scene_load::スキン素材へ変換する(&シーン)?;

    let レンダラー = レンダラー::生成する(
        表示ハンドル,
        ウィンドウハンドル,
        寸法,
        シェーダー束,
        &頂点一覧,
        &インデックス一覧,
        マテリアル,
        スキン素材,
        ポスト処理有効,
    )?;

    ホットリローダー.アセット監視を設定する(カタログ, アセットID::生成する(シーン名)?, &シーン.参照ファイル一覧);

    let アニメーション = アニメーション再生::生成する(シーン.スキン, シーン.アニメーション一覧);
    let 開発ui = 開発UI::生成する(&window, 開発ui初期有効);
    Ok((window, レンダラー, 開発ui, アニメーション))
}
