//! `--frames`スモークモードの自己操作・ホットリロード検証・ピクセル判定シナリオ。
//! DoDの「標準サンプルが表示される」「アセット変更が反映される」
//! 「リサイズ・最小化で落ちない」をウィンドウの自己操作とピクセル読み戻しで機械検証する。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断9」「判断22」。

mod asset_rewrite;
pub(crate) mod material_reload;
mod pixel_judgment;
mod plan;
mod rebuild_plan;
mod shader_rewrite;
mod window_operation;

use winit::window::Window;

use crate::cli::粒子表示モード;

/// 書き換えもピクセル判定も持たず、判定を検収側のxtaskが読み戻し画像と計数で行うシーンの名前の接頭辞。
/// 植生は`instance-*`、`prop_`は`prop-draw`と`village-draw`、`multi_material`は`multi-material-draw`が判定する。
/// 既定の計画は`quad`のホットリロード検証であり、監視対象のシェーダーとquadの実行時形式を書き換えるため、
/// これらをその計画へ落とすとリポジトリのシェーダーと既存の生成物が書き換わる。
const 読み戻しだけの検収シーンの接頭辞一覧: [&str; 3] = ["vegetation", "prop_", "multi_material"];
/// 群が両方の視錐台から外れる検収シーン(`cargo xtask cloth-empty`)。中身は植生の検収シーンと同じだが、
/// 既定カメラと既定の影範囲を選ばせるために名前を接頭辞から外してあるため、ここで個別に同じ計画へ振る。
const 両視錐台外の群シーン: &str = "instance_all_culled";

pub(crate) use asset_rewrite::アセットを書き換える;
pub(crate) use pixel_judgment::{アニメーション差分を判定する, ピクセルを判定する};
pub(crate) use rebuild_plan::ウィンドウ再構築計画;
pub(crate) use shader_rewrite::シェーダーを書き換える;

/// フレーム番号に応じて、このフレームで行う自己操作・検証を表す。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum スモークアクション {
    通常描画,
    リサイズ,
    最小化,
    復帰,
    シェーダー書き換え,
    アセット書き換え,
    /// 材質差し替えステージ: このフレームの読み戻し画像を差し替え前の絵として別名で書き出す。
    材質差し替え前ダンプ,
    初期色判定,
    アセット反映後判定,
    最終判定,
    ヘルメット判定,
    粒子判定,
    開発UI判定,
    シャドウ判定,
    複数描画対象判定,
    /// foxステージ: このフレームの読み戻し画像を差分判定の基準として保存する(判断45)。
    フォックス基準保存,
    /// foxステージ: 保存済み基準とこのフレームの読み戻し画像を比較し、アニメーションで絵が動いたことを判定する。
    フォックス差分判定,
}

/// `布有効`なら差分計画(布は厳密ピクセル判定と両立しないため。判断55・56)、`開発ui有効`なら
/// devui計画、粒子トイならparticles計画、表面流ならGPU計測だけ、読み戻しだけの検収シーンの接頭辞で始まるならその計画、
/// シーン名がhelmet/shadow_scene/foxなら各計画、それ以外(既定"quad")ならquad計画で判定する(判断29・34・37・45)。
pub(crate) fn 判定する(
    現在フレーム: u32,
    総フレーム数: u32,
    シーン名: &str,
    粒子表示: 粒子表示モード,
    開発ui有効: bool,
    布有効: bool,
    描画対象数: Option<crate::cli::描画対象数>,
) -> スモークアクション {
    if 描画対象数.map(crate::cli::描画対象数::usize値) == Some(2) {
        plan::複数描画対象計画(現在フレーム, 総フレーム数)
    } else if 布有効 {
        plan::フォックス計画(現在フレーム)
    } else if 開発ui有効 {
        plan::devui計画(現在フレーム, 総フレーム数)
    } else if matches!(
        粒子表示,
        粒子表示モード::表面流 | 粒子表示モード::Sph512 | 粒子表示モード::Sph1024 | 粒子表示モード::Sph2048
    ) {
        スモークアクション::通常描画
    } else if 粒子表示 == 粒子表示モード::粒子トイ {
        plan::particles計画(現在フレーム, 総フレーム数)
    } else if material_reload::このシーンか(シーン名) {
        material_reload::計画(現在フレーム, 総フレーム数)
    } else if シーン名 == 両視錐台外の群シーン || 読み戻しだけの検収シーンの接頭辞一覧.iter().any(|接頭辞| シーン名.starts_with(接頭辞))
    {
        plan::読み戻しだけの計画()
    } else if シーン名 == "helmet" {
        plan::helmet計画(現在フレーム, 総フレーム数)
    } else if シーン名 == "shadow_scene" {
        plan::shadow計画(現在フレーム, 総フレーム数)
    } else if シーン名 == "fox" {
        plan::フォックス計画(現在フレーム)
    } else {
        plan::quad計画(現在フレーム, 総フレーム数)
    }
}

pub(crate) fn window自己操作を適用する(window: &Window, アクション: スモークアクション) {
    window_operation::適用する(window, アクション);
}
