//! `--frames`スモークモードの自己操作・ホットリロード検証・ピクセル判定シナリオ。
//! DoDの「標準サンプルが表示される」「アセット変更が反映される」
//! 「リサイズ・最小化で落ちない」をウィンドウの自己操作とピクセル読み戻しで機械検証する。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断9」「判断22」。

mod asset_rewrite;
mod pixel_judgment;
mod plan;
mod shader_rewrite;
mod window_operation;

use winit::window::Window;

pub(crate) use asset_rewrite::アセットを書き換える;
pub(crate) use pixel_judgment::{アニメーション差分を判定する, ピクセルを判定する};
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
    初期色判定,
    アセット反映後判定,
    最終判定,
    ヘルメット判定,
    粒子判定,
    開発UI判定,
    シャドウ判定,
    /// foxステージ: このフレームの読み戻し画像を差分判定の基準として保存する(判断45)。
    フォックス基準保存,
    /// foxステージ: 保存済み基準とこのフレームの読み戻し画像を比較し、アニメーションで絵が動いたことを判定する。
    フォックス差分判定,
}

/// `開発ui有効`ならdevui計画、`粒子有効`ならparticles計画、`シーン名`が"helmet"なら
/// helmet計画、"shadow_scene"ならshadow計画、それ以外(既定"quad")ならquad計画で
/// 判定する(判断29・判断34・判断37)。
pub(crate) fn 判定する(
    現在フレーム: u32,
    総フレーム数: u32,
    シーン名: &str,
    粒子有効: bool,
    開発ui有効: bool,
) -> スモークアクション {
    if 開発ui有効 {
        plan::devui計画(現在フレーム, 総フレーム数)
    } else if 粒子有効 {
        plan::particles計画(現在フレーム, 総フレーム数)
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
