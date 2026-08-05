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
    /// 段差の走査では撮影ごとの最後のフレームが対象であり、そちらは走査が答える。
    /// このフレームはスモーク判定の代わりに読み戻しとファイル書き出しを行う。
    pub(super) fn ダンプ対象フレームか(&self, アクション: スモークアクション) -> bool {
        if self.天空.段差走査で読み戻すフレームか(self.現在フレーム) {
            return true;
        }
        let 起動モード::スモーク実行 { フレーム数 } = self.起動モード else {
            return false;
        };
        let 採るフレームか = self.現在フレーム + 1 == フレーム数 || アクション == スモークアクション::差し替え前ダンプ;
        self.フレームダンプ先.書き出すか() && 採るフレームか
    }

    pub(super) fn 読み戻してダンプする(
        &mut self,
        描画入力: blitz_render::フレーム描画入力<'_>,
        視点情報: &フレーム視点,
        アクション: スモークアクション,
    ) -> Result<描画の到達, 起動エラー> {
        if let Some(ダンプ先) = 段差走査の書き出し先(self) {
            return hdr_dump::読み戻して書き出す(self, 描画入力, &ダンプ先);
        }
        match self.フレームダンプ先.clone() {
            フレームダンプ指定::指定なし => Ok(描画の到達::届かなかった),
            フレームダンプ指定::提示画像を書き出す { 基準名 } => {
                presentation_dump::読み戻して書き出す(self, 描画入力, 視点情報, &書き出し先を選ぶ(&基準名, アクション))
            }
            フレームダンプ指定::圧縮前のHDRを書き出す { 基準名 } => {
                hdr_dump::読み戻して書き出す(self, 描画入力, &書き出し先を選ぶ(&基準名, アクション))
            }
        }
    }
}

/// 段差の走査で読み戻すフレームの書き出し先。ベース名の後ろへ境界の識別と側を足すため、対の2枚が並んで残る。
/// 走査でないフレームでは無い。
///
/// 書き出し先の組み立てを走査の側に持たせないのは、ベース名を持つのが`--dump-hdr-frame`の指定であり、
/// その指定を読むのがこの入口だからである。走査はファイル名に足す語だけを答える。
fn 段差走査の書き出し先(アプリ: &アプリ) -> Option<PathBuf> {
    let 撮影 = アプリ.天空.このフレームの撮影(アプリ.現在フレーム)?;
    if !アプリ.天空.段差走査で読み戻すフレームか(アプリ.現在フレーム) {
        return None;
    }
    let フレームダンプ指定::圧縮前のHDRを書き出す { 基準名 } = &アプリ.フレームダンプ先 else {
        return None;
    };
    let 名前 = 基準名
        .file_name()
        .map_or_else(|| "step".to_string(), |名前| 名前.to_string_lossy().into_owned());
    Some(基準名.with_file_name(format!("{名前}{}", 撮影.ファイル名の後置き())))
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
