//! `--dump-frame <ベース名>` の書き出し: 読み戻し画像を
//! `<ベース名>.raw`(RGBA8連結)と`<ベース名>.size`(幅 高さ)へ保存する。
//! 親エージェントの検収工程「絵の目視監査」用(経緯: M6の影バグはvalidation・
//! ピクセル判定の両方をすり抜け、絵を見ることでのみ検出できた)。
//! 空の代表画素の照合は`sky_pixel_check`が持つ。

mod sky_pixel_check;

use std::path::{Path, PathBuf};

use blitz_render::読み戻し画像;

use super::draw_dispatch::描画の到達;
use super::frame::フレーム視点;
use super::アプリ;
use crate::cli::起動モード;
use crate::error::起動エラー;
use crate::smoke::スモークアクション;

impl アプリ {
    /// `--dump-frame`が指定され、かつ`--frames`の最終フレームか差し替え前の絵を採るフレームなら真。
    /// このフレームはスモーク判定の代わりに読み戻しとファイル書き出しを行う。
    pub(super) fn ダンプ対象フレームか(&self, アクション: スモークアクション) -> bool {
        let 起動モード::スモーク実行 { フレーム数 } = self.起動モード else {
            return false;
        };
        let 採るフレームか = self.現在フレーム + 1 == フレーム数 || アクション == スモークアクション::材質差し替え前ダンプ;
        self.フレームダンプ先.is_some() && 採るフレームか
    }

    pub(super) fn 読み戻してダンプする(
        &mut self,
        描画入力: blitz_render::フレーム描画入力<'_>,
        視点情報: &フレーム視点,
        アクション: スモークアクション,
    ) -> Result<描画の到達, 起動エラー> {
        let Some(ダンプ先) = self.フレームダンプ先.as_deref().map(|基準| 書き出し先を選ぶ(基準, アクション)) else {
            return Ok(描画の到達::届かなかった);
        };
        let Some(レンダラー) = &mut self.レンダラー else {
            return Ok(描画の到達::届かなかった);
        };
        match レンダラー.一フレーム描画して読み戻す(描画入力)? {
            blitz_render::読み戻し結果::読み戻した(画像) => {
                書き出す(&画像, &ダンプ先)?;
                sky_pixel_check::照合する(self, &画像, 視点情報);
                Ok(描画の到達::提示した)
            }
            blitz_render::読み戻し結果::見送った(理由) => Err(起動エラー::フレームダンプ失敗(format!(
                "ダンプ対象フレームで描画が見送られた: {理由:?}"
            ))),
        }
    }
}

/// 差し替え前のフレームは`<ベース名>_before`へ書き出す。同じ実行の2枚を検収側が同じ読み手で読めるよう、
/// 拡張子の付け方は最終フレームと同じにする。
fn 書き出し先を選ぶ(基準: &Path, アクション: スモークアクション) -> PathBuf {
    if アクション != スモークアクション::材質差し替え前ダンプ {
        return 基準.to_path_buf();
    }
    let 名前 = 基準
        .file_name()
        .map_or_else(|| "frame".to_string(), |名前| 名前.to_string_lossy().into_owned());
    基準.with_file_name(format!("{名前}_before"))
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
    let sizeパス = ベース名.with_extension("size");
    std::fs::write(&rawパス, バイト列).map_err(|誤り| 起動エラー::フレームダンプ失敗(format!("{}: {誤り}", rawパス.display())))?;
    std::fs::write(&sizeパス, format!("{幅} {高さ}\n"))
        .map_err(|誤り| 起動エラー::フレームダンプ失敗(format!("{}: {誤り}", sizeパス.display())))?;
    println!("[dump-frame] 書き出した: {}", rawパス.display());
    Ok(())
}
