//! resumed時のウィンドウ・レンダラー生成。カタログ構築・シーン読込・
//! アセットホットリロード監視の初期設定もここで行う。

mod window_create;

use blitz_render::{ウィンドウ寸法, レンダラー, 実表示計測要求};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

use super::animation_state::アニメーション再生;
use super::scene_load;
use crate::embedded_shaders;
use crate::error::起動エラー;
use crate::hot_reload::ホットリローダー;
use crate::overlay_ui::画面へ重ねるUI;

/// resumed時に構築してアプリのフィールドへ格納する一式。
type 起動一式 = (
    Window,
    レンダラー,
    画面へ重ねるUI,
    Option<アニメーション再生>,
    Option<super::cloth_setup::布プリセット>,
    scene_load::束の登録一式,
);

/// ウィンドウを生成し、そのハンドルからレンダラー・開発用UIを生成する。
///
/// 前提: 戻り値のタプルはこの順でアプリ構造体のフィールドへ格納され、
/// windowがレンダラーより先にDropされないことをフィールド宣言順で保証する。
#[allow(clippy::too_many_arguments)]
pub(super) fn ウィンドウとレンダラーを作る(
    event_loop: &ActiveEventLoop,
    シーン: &crate::cli::起動時シーン,
    アセットルート: &std::path::Path,
    描画対象の並べ方: crate::cli::描画対象の並べ方,
    ホットリローダー: &mut ホットリローダー,
    粒子表示: crate::cli::粒子表示モード,
    空中遠近合成: crate::cli::空中遠近合成指定,
    開発ui初期有効: bool,
    フレーム構成: blitz_render::フレーム構成,
    布モード: crate::cli::布モード,
    実表示計測要求: 実表示計測要求,
    大域平行移動: blitz_math::大域ワールド位置,
    影の一辺解像度: blitz_render::cascade::影の一辺解像度,
    照明問い合わせ契約: blitz_render::indirect_lighting::照明問い合わせ契約,
    自動露出の設定: blitz_render::auto_exposure::自動露出の設定,
    局所可視性の描画設定: blitz_render::local_visibility::局所可視性の描画設定,
    時間再構成の描画設定: blitz_render::temporal_reconstruction::時間再構成の描画設定,
    ゲーム配線: &mut crate::game::ゲーム配線,
) -> Result<起動一式, 起動エラー> {
    let window = window_create::生成する(event_loop)?;
    let 表示ハンドル = window.display_handle()?.as_raw();
    let ウィンドウハンドル = window.window_handle()?.as_raw();
    let 物理寸法 = window.inner_size();
    let 寸法 = ウィンドウ寸法::生成する(物理寸法.width, 物理寸法.height);
    let シェーダー束 = embedded_shaders::埋め込みシェーダー束を生成する(粒子表示, 空中遠近合成)?;
    let 粒子素材 = super::particle_setup::素材を作る(粒子表示)?;

    let カタログ = scene_load::カタログを構築して高さ場を据える(アセットルート, ゲーム配線)?;
    let (シーンデータ, mut 描画入力) =
        scene_load::シーンを読み込んで変換する(&カタログ, シーン.安定id(), 描画対象の並べ方, 大域平行移動)?;
    // 動く個体の宣言は束の読込より前でなければならない。宣言した個体だけがフレームスロットごとのバッファを持ち、そのバッファを読込時のディスクリプタが結ぶ。
    ゲーム配線.束の描画シーン素材へ動く個体を宣言する(scene_load::起動時シーンの束ID, &mut 描画入力.描画シーン)?;
    if 描画対象の並べ方.件数.is_some() {
        crate::reports::composition::描画対象構成を表示する(描画入力.描画シーン.描画対象数());
        crate::reports::composition::フレーム構成を表示する(&フレーム構成);
    }
    let スキン素材 = scene_load::スキン素材へ変換する(&シーンデータ)?;
    let 布 = super::cloth_setup::布モードから構築する(布モード, &描画入力.描画シーン)?;
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
        影の一辺解像度,
        照明問い合わせ契約,
        自動露出の設定,
        局所可視性の描画設定,
        時間再構成の描画設定,
    )?;

    ホットリローダー.アセット監視を設定する(カタログ, シーン.安定id().clone(), &シーンデータ.参照ファイル一覧);

    let アニメーション = アニメーション再生::生成する(シーンデータ.スキン, シーンデータ.アニメーション一覧);
    let 画面へ重ねるui = 画面へ重ねるUI::生成する(&window, 開発ui初期有効);
    Ok((window, レンダラー, 画面へ重ねるui, アニメーション, 布プリセット, 描画入力.登録一式))
}
