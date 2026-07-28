//! resumed時のウィンドウ・レンダラー生成。カタログ構築・シーン読込・
//! アセットホットリロード監視の初期設定もここで行う。

mod window_create;

use blitz_engine::アセットID;
use blitz_render::{ウィンドウ寸法, レンダラー, 実表示計測要求};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

use super::animation_state::アニメーション再生;
use super::scene_load;
use crate::dev_ui::開発UI;
use crate::embedded_shaders;
use crate::error::起動エラー;
use crate::hot_reload::ホットリローダー;

/// resumed時に構築してアプリのフィールドへ格納する一式。
type 起動一式 = (
    Window,
    レンダラー,
    開発UI,
    Option<アニメーション再生>,
    Option<super::cloth_setup::布プリセット>,
    Vec<super::visibility::群可視材料の登録>,
);

/// ウィンドウを生成し、そのハンドルからレンダラー・開発用UIを生成する。
///
/// 前提: 戻り値のタプルはこの順でアプリ構造体のフィールドへ格納され、
/// windowがレンダラーより先にDropされないことをフィールド宣言順で保証する。
#[allow(clippy::too_many_arguments)]
pub(super) fn ウィンドウとレンダラーを作る(
    event_loop: &ActiveEventLoop,
    シーン名: &str,
    アセットルート: &std::path::Path,
    描画対象数: Option<crate::cli::描画対象数>,
    ホットリローダー: &mut ホットリローダー,
    粒子表示: crate::cli::粒子表示モード,
    開発ui初期有効: bool,
    フレーム構成: blitz_render::フレーム構成,
    布モード: crate::cli::布モード,
    実表示計測要求: 実表示計測要求,
    大域平行移動: blitz_math::大域ワールド位置,
) -> Result<起動一式, 起動エラー> {
    let window = window_create::生成する(event_loop)?;
    let 表示ハンドル = window.display_handle()?.as_raw();
    let ウィンドウハンドル = window.window_handle()?.as_raw();
    let 物理寸法 = window.inner_size();
    let 寸法 = ウィンドウ寸法::生成する(物理寸法.width, 物理寸法.height);
    let シェーダー束 = embedded_shaders::埋め込みシェーダー束を生成する(粒子表示)?;
    let 粒子素材 = super::particle_setup::素材を作る(粒子表示)?;

    let カタログ = scene_load::カタログを構築する(アセットルート)?;
    let (シーン, 描画入力) = scene_load::シーンを読み込んで変換する(&カタログ, シーン名, 描画対象数, 大域平行移動)?;
    if 描画対象数.is_some() {
        crate::reports::composition::描画対象構成を表示する(描画入力.描画シーン.描画対象数());
        crate::reports::composition::フレーム構成を表示する(&フレーム構成);
    }
    let スキン素材 = scene_load::スキン素材へ変換する(&シーン)?;
    let 布 = 布を構築する(布モード, &描画入力.描画シーン)?;
    let (布素材, 布プリセット) = match 布 {
        Some((素材, プリセット)) => (Some(素材), Some(プリセット)),
        None => (None, None),
    };

    let レンダラー = レンダラー::生成する(
        表示ハンドル,
        ウィンドウハンドル,
        寸法,
        シェーダー束,
        描画入力.描画シーン,
        スキン素材,
        布素材,
        粒子素材,
        フレーム構成,
        実表示計測要求,
    )?;

    ホットリローダー.アセット監視を設定する(カタログ, アセットID::生成する(シーン名)?, &シーン.参照ファイル一覧);

    let アニメーション = アニメーション再生::生成する(シーン.スキン, シーン.アニメーション一覧);
    let 開発ui = 開発UI::生成する(&window, 開発ui初期有効);
    Ok((window, レンダラー, 開発ui, アニメーション, 布プリセット, 描画入力.可視材料一覧))
}

fn 布を構築する(
    布モード: crate::cli::布モード,
    描画シーン: &blitz_render::描画シーン素材,
) -> Result<Option<(blitz_render::布素材, super::cloth_setup::布プリセット)>, 起動エラー> {
    match 布モード {
        crate::cli::布モード::なし => Ok(None),
        crate::cli::布モード::吊るし布 => Ok(Some(super::cloth_setup::吊るし布を構築する()?)),
        crate::cli::布モード::マント => Ok(Some(super::cloth_setup::マントを構築する(
            描画シーン.先頭の描画対象().最詳細段の頂点一覧(),
        )?)),
    }
}
