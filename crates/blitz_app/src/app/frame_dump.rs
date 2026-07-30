//! `--dump-frame <ベース名>` の書き出し: 最終フレームの読み戻し画像を
//! `<ベース名>.raw`(RGBA8連結)と`<ベース名>.size`(幅 高さ)へ保存する。
//! 親エージェントの検収工程「絵の目視監査」用(経緯: M6の影バグはvalidation・
//! ピクセル判定の両方をすり抜け、絵を見ることでのみ検出できた)。

use std::path::Path;

use blitz_render::読み戻し画像;

use super::frame::フレーム視点;
use super::アプリ;
use crate::cli::起動モード;
use crate::error::起動エラー;

impl アプリ {
    /// `--dump-frame`が指定され、かつ`--frames`の最終フレームなら真。このフレームはスモーク判定の代わりに読み戻しとファイル書き出しを行う。
    pub(super) fn ダンプ対象フレームか(&self) -> bool {
        let 起動モード::スモーク実行 { フレーム数 } = self.起動モード else {
            return false;
        };
        self.フレームダンプ先.is_some() && self.現在フレーム + 1 == フレーム数
    }

    pub(super) fn 読み戻してダンプする(
        &mut self,
        描画入力: blitz_render::フレーム描画入力<'_>,
        視点情報: &フレーム視点,
    ) -> Result<(), 起動エラー> {
        let Some(レンダラー) = &mut self.レンダラー else {
            return Ok(());
        };
        let Some(ダンプ先) = &self.フレームダンプ先 else {
            return Ok(());
        };
        match レンダラー.一フレーム描画して読み戻す(描画入力)? {
            blitz_render::読み戻し結果::読み戻した(画像) => {
                書き出す(&画像, ダンプ先)?;
                self.空代表画素を照合する(&画像, 視点情報);
                Ok(())
            }
            blitz_render::読み戻し結果::見送った(理由) => Err(起動エラー::フレームダンプ失敗(format!(
                "最終フレームで描画が見送られた: {理由:?}"
            ))),
        }
    }

    /// `--report-sky-pixel`が指定されていなければ何もしない。空を持たない世界や大気LUT資源を持たない
    /// フレーム構成では、指定があっても照合の材料が無い旨を1行だけ出す。
    fn 空代表画素を照合する(&self, 画像: &読み戻し画像, 視点情報: &フレーム視点) {
        if self.空の代表画素報告.is_empty() {
            return;
        }
        let Some(条件) = self.天空.再現条件() else {
            println!("空代表画素 照合できなかった 理由=このフレーム構成に大気LUTの材料が無い");
            return;
        };
        let 逆ビュー射影行列 = 視点情報.ビュー射影.逆変換する().gpu境界用列優先配列();
        crate::reports::sky_pixel::代表画素を照合する(&crate::reports::sky_pixel::照合の材料 {
            条件,
            画像,
            逆ビュー射影行列,
            // カメラ相対フレームの原点はこのフレームのカメラ自身の大域位置に置く(`frame::視点を求める`参照)ため、
            // カメラのカメラ相対位置は常に原点である。
            カメラ相対位置: [0.0, 0.0, 0.0],
            画素一覧: &self.空の代表画素報告,
        });
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
    let sizeパス = ベース名.with_extension("size");
    std::fs::write(&rawパス, バイト列).map_err(|誤り| 起動エラー::フレームダンプ失敗(format!("{}: {誤り}", rawパス.display())))?;
    std::fs::write(&sizeパス, format!("{幅} {高さ}\n"))
        .map_err(|誤り| 起動エラー::フレームダンプ失敗(format!("{}: {誤り}", sizeパス.display())))?;
    println!("[dump-frame] 書き出した: {}", rawパス.display());
    Ok(())
}
