//! 提示画像の読み戻しと書き出しの工程。受け取るのはアプリと1フレームの描画入力と視点とダンプ先、返すのはそのフレームが提示まで届いたかどうかである。
//!
//! 書き出す形式は`<ベース名>.raw`(RGBA8連結)と`<ベース名>.size`(幅 高さ)である。
//! 代表画素の照合をこの工程が呼ぶのは、どの照合も提示画像の8ビット値を材料にしており、読み戻した画像がここにしか無いためである。

use std::path::Path;

use blitz_render::読み戻し画像;

use super::super::draw_dispatch::描画の到達;
use super::super::frame::フレーム視点;
use super::super::アプリ;
use super::{cluster_assignment_check, indirect_probe_check, sky_pixel_check, 寸法を書く};
use crate::error::起動エラー;

pub(super) fn 読み戻して書き出す(
    アプリ: &mut アプリ,
    描画入力: blitz_render::フレーム描画入力<'_>,
    視点情報: &フレーム視点,
    ダンプ先: &Path,
) -> Result<描画の到達, 起動エラー> {
    let Some(レンダラー) = &mut アプリ.レンダラー else {
        return Ok(描画の到達::届かなかった);
    };
    match レンダラー.一フレーム描画して読み戻す(描画入力)? {
        blitz_render::読み戻し結果::読み戻した(画像) => {
            書き出す(&画像, ダンプ先)?;
            sky_pixel_check::照合する(アプリ, &画像, 視点情報);
            indirect_probe_check::照合する(アプリ, &画像, 視点情報);
            cluster_assignment_check::報告する(アプリ, 視点情報);
            Ok(描画の到達::提示した)
        }
        blitz_render::読み戻し結果::見送った(理由) => Err(起動エラー::フレームダンプ失敗(format!(
            "ダンプ対象フレームで描画が見送られた: {理由:?}"
        ))),
    }
}

fn 書き出す(画像: &読み戻し画像, ベース名: &Path) -> Result<(), 起動エラー> {
    let 幅 = 画像.幅();
    let 高さ = 画像.高さ();
    let mut バイト列 = Vec::with_capacity(usize::try_from(u64::from(幅) * u64::from(高さ) * 4).unwrap_or(0));
    for y in 0..高さ {
        for x in 0..幅 {
            let ピクセル = 画像.ピクセル(x, y).unwrap_or([0, 0, 0, 0]);
            バイト列.extend_from_slice(&ピクセル);
        }
    }

    let rawパス = ベース名.with_extension("raw");
    std::fs::write(&rawパス, バイト列).map_err(|誤り| 起動エラー::フレームダンプ失敗(format!("{}: {誤り}", rawパス.display())))?;
    寸法を書く(幅, 高さ, ベース名)?;
    println!("[dump-frame] 書き出した: {}", rawパス.display());
    Ok(())
}
