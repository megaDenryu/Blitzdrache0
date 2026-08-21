//! resumed時のウィンドウ・レンダラー生成。カタログ構築・シーン読込・
//! アセットホットリロード監視の初期設定もここで行う。材料と一式の形は`materials`が持つ。
//!
//! 段階を組み立てるビルダーを別に立てないのは、部品が揃ったことを検査して完成させる口が`レンダラー::生成する`
//! そのものであり、その口をblitz_render側が持つためである。ここが半端な状態を引き受ける型を作ると、
//! 同じ検査が2箇所に並ぶ。

mod materials;
mod window_create;

use blitz_render::{ウィンドウ寸法, レンダラー};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::event_loop::ActiveEventLoop;

pub(in crate::app) use materials::{画面の作り方, 読み込む世界の材料, 起動時に組み上げた一式};

use super::animation_state::アニメーション再生;
use super::scene_load;
use crate::embedded_shaders;
use crate::error::起動エラー;
use crate::hot_reload::ホットリローダー;
use crate::overlay_ui::画面へ重ねるUI;

/// ウィンドウを生成し、そのハンドルからレンダラー・開発用UIを生成する。
pub(super) fn ウィンドウとレンダラーを作る(
    event_loop: &ActiveEventLoop,
    世界: &読み込む世界の材料<'_>,
    作り方: &画面の作り方,
    ホットリローダー: &mut ホットリローダー,
    ゲーム配線: &mut crate::game::ゲーム配線,
) -> Result<起動時に組み上げた一式, 起動エラー> {
    let window = window_create::生成する(event_loop)?;
    let 表示ハンドル = window.display_handle()?.as_raw();
    let ウィンドウハンドル = window.window_handle()?.as_raw();
    let 物理寸法 = window.inner_size();
    let 寸法 = ウィンドウ寸法::生成する(物理寸法.width, 物理寸法.height);
    let シェーダー束 = embedded_shaders::埋め込みシェーダー束を生成する(作り方.粒子表示, 作り方.空中遠近合成)?;
    let 粒子素材 = super::particle_setup::素材を作る(作り方.粒子表示)?;

    let カタログ = scene_load::カタログを構築して高さ場を据える(世界.アセットの置き場, ゲーム配線)?;
    let (シーンデータ, mut 描画入力) = scene_load::シーンを読み込んで変換する(
        &カタログ,
        世界.シーン.安定id(),
        世界.描画対象の並べ方,
        世界.大域平行移動,
        世界.チャンク一辺,
    )?;
    let 遠景 = scene_load::遠景を読み込んで変換する(&カタログ, 世界.大域平行移動, 世界.チャンク一辺)?;
    // 動く個体の宣言は束の読込より前でなければならない。宣言した個体だけがフレームスロットごとのバッファを持ち、そのバッファを読込時のディスクリプタが結ぶ。
    ゲーム配線.束の描画シーン素材へ動く個体を宣言する(scene_load::起動時シーンの束ID, &mut 描画入力.描画シーン)?;
    if 世界.描画対象の並べ方.件数.is_some() {
        crate::reports::composition::描画対象構成を表示する(描画入力.描画シーン.描画対象数());
        crate::reports::composition::フレーム構成を表示する(&作り方.フレーム構成);
    }
    let スキン素材 = scene_load::スキン素材へ変換する(&シーンデータ)?;
    let 布 = super::cloth_setup::布モードから構築する(世界.布モード, &描画入力.描画シーン)?;
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
        作り方.フレーム構成,
        作り方.実表示計測要求,
        作り方.影の一辺解像度,
        作り方.照明問い合わせ契約,
        作り方.自動露出の設定,
        作り方.局所可視性の描画設定,
        作り方.時間再構成の描画設定,
    )?;

    ホットリローダー.アセット監視を設定する(世界.アセットの置き場.clone(), カタログ, 世界.シーン.安定id().clone());

    Ok(起動時に組み上げた一式 {
        画面へ重ねるui: 画面へ重ねるUI::生成する(&window, 作り方.開発ui初期有効),
        アニメーション: アニメーション再生::生成する(シーンデータ.スキン, シーンデータ.アニメーション一覧),
        window,
        レンダラー,
        布プリセット,
        登録一式: 描画入力.登録一式,
        遠景,
    })
}
