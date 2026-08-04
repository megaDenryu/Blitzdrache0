//! 明るさの圧縮前のHDR中間画像の読み戻しと書き出しの工程。受け取るのはアプリと1フレームの描画入力とダンプ先、
//! 返すのはそのフレームが提示まで届いたかどうかである。
//!
//! 書き出す形式は`<ベース名>.hdr32`(RGBA単精度リトルエンディアン連結、行優先)と`<ベース名>.size`(幅 高さ)である。
//! 単精度で書くのは、半精度のビット列の意味づけを知るのがGPU境界の側だけであり、
//! 読み手が同じ復号をもう1つ持つと丸めの食い違いが検収の差に化けるためである。
//!
//! 代表画素の照合を呼ばないのは、どの照合も提示画像の8ビット値をCPU正本と突き合わせる工程であり、
//! 圧縮前の値に対する期待を1つも持たないためである。

use std::path::Path;

use blitz_render::HDR読み戻し画像;

use super::super::draw_dispatch::描画の到達;
use super::super::アプリ;
use super::寸法を書く;
use crate::error::起動エラー;

pub(super) fn 読み戻して書き出す(
    アプリ: &mut アプリ,
    描画入力: blitz_render::フレーム描画入力<'_>,
    ダンプ先: &Path,
) -> Result<描画の到達, 起動エラー> {
    let Some(レンダラー) = &mut アプリ.レンダラー else {
        return Ok(描画の到達::届かなかった);
    };
    match レンダラー.一フレーム描画して圧縮前のhdrを読み戻す(描画入力)? {
        blitz_render::読み戻し結果::読み戻した(画像) => {
            書き出す(&画像, ダンプ先)?;
            Ok(描画の到達::提示した)
        }
        blitz_render::読み戻し結果::見送った(理由) => Err(起動エラー::フレームダンプ失敗(format!(
            "ダンプ対象フレームで描画が見送られた: {理由:?}"
        ))),
    }
}

fn 書き出す(画像: &HDR読み戻し画像, ベース名: &Path) -> Result<(), 起動エラー> {
    let 成分列 = 画像.成分列();
    let mut バイト列 = Vec::with_capacity(成分列.len() * 4);
    for 成分 in 成分列 {
        バイト列.extend_from_slice(&成分.to_le_bytes());
    }

    let hdrパス = ベース名.with_extension("hdr32");
    std::fs::write(&hdrパス, バイト列).map_err(|誤り| 起動エラー::フレームダンプ失敗(format!("{}: {誤り}", hdrパス.display())))?;
    寸法を書く(画像.幅(), 画像.高さ(), ベース名)?;
    println!("[dump-hdr-frame] 書き出した: {}", hdrパス.display());
    Ok(())
}
