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
pub(crate) use pixel_judgment::ピクセルを判定する;
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
}

/// `シーン名`が"helmet"ならhelmet計画、それ以外(既定"quad")ならquad計画で判定する。
pub(crate) fn 判定する(現在フレーム: u32, 総フレーム数: u32, シーン名: &str) -> スモークアクション {
    if シーン名 == "helmet" {
        plan::helmet計画(現在フレーム, 総フレーム数)
    } else {
        plan::quad計画(現在フレーム, 総フレーム数)
    }
}

pub(crate) fn window自己操作を適用する(window: &Window, アクション: スモークアクション) {
    window_operation::適用する(window, アクション);
}
