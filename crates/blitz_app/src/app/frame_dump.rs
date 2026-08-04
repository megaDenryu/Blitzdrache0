//! `--dump-frame` `--dump-hdr-frame` の書き出しの入口。担うのは「このフレームがダンプ対象か」と「どの画像をどのベース名で書き出すか」の振り分けだけであり、外部形式への写しは対象ごとの工程が持つ。
//! 親エージェントの検収工程「絵の目視監査」用(経緯: M6の影バグはvalidation・
//! ピクセル判定の両方をすり抜け、絵を見ることでのみ検出できた)。
//! 提示画像の書き出しと照合は`presentation_dump`、圧縮前のHDRの書き出しは`hdr_dump`が持つ。

mod hdr_dump;
mod indirect_probe_check;
mod presentation_dump;
mod sky_pixel_check;

use std::path::{Path, PathBuf};

use super::draw_dispatch::描画の到達;
use super::frame::フレーム視点;
use super::アプリ;
use crate::cli::{フレームダンプ指定, 起動モード};
use crate::error::起動エラー;
use crate::smoke::スモークアクション;

impl アプリ {
    /// フレームダンプが指定され、かつ`--frames`の最終フレームか差し替え前の絵を採るフレームなら真。
    /// このフレームはスモーク判定の代わりに読み戻しとファイル書き出しを行う。
    pub(super) fn ダンプ対象フレームか(&self, アクション: スモークアクション) -> bool {
        let 起動モード::スモーク実行 { フレーム数 } = self.起動モード else {
            return false;
        };
        let 採るフレームか = self.現在フレーム + 1 == フレーム数 || アクション == スモークアクション::差し替え前ダンプ;
        self.フレームダンプ先.is_some() && 採るフレームか
    }

    pub(super) fn 読み戻してダンプする(
        &mut self,
        描画入力: blitz_render::フレーム描画入力<'_>,
        視点情報: &フレーム視点,
        アクション: スモークアクション,
    ) -> Result<描画の到達, 起動エラー> {
        let Some(指定) = self.フレームダンプ先.clone() else {
            return Ok(描画の到達::届かなかった);
        };
        let ダンプ先 = 書き出し先を選ぶ(指定.基準名(), アクション);
        match 指定 {
            フレームダンプ指定::提示画像を書き出す { .. } => {
                presentation_dump::読み戻して書き出す(self, 描画入力, 視点情報, &ダンプ先)
            }
            フレームダンプ指定::圧縮前のHDRを書き出す { .. } => hdr_dump::読み戻して書き出す(self, 描画入力, &ダンプ先),
        }
    }
}

/// 差し替え前のフレームは`<ベース名>_before`へ書き出す。同じ実行の2枚を検収側が同じ読み手で読めるよう、
/// 拡張子の付け方は最終フレームと同じにする。
fn 書き出し先を選ぶ(基準: &Path, アクション: スモークアクション) -> PathBuf {
    if アクション != スモークアクション::差し替え前ダンプ {
        return 基準.to_path_buf();
    }
    let 名前 = 基準
        .file_name()
        .map_or_else(|| "frame".to_string(), |名前| 名前.to_string_lossy().into_owned());
    基準.with_file_name(format!("{名前}_before"))
}

/// 寸法ファイルは対象によらず同じ形式で書く。検収側が画素の並びを読むためにまず要るのが幅と高さであり、
/// 対象ごとに読み手を分けると同じ2数を2通りで読むことになる。
fn 寸法を書く(幅: u32, 高さ: u32, ベース名: &Path) -> Result<(), 起動エラー> {
    let sizeパス = ベース名.with_extension("size");
    std::fs::write(&sizeパス, format!("{幅} {高さ}\n"))
        .map_err(|誤り| 起動エラー::フレームダンプ失敗(format!("{}: {誤り}", sizeパス.display())))
}
