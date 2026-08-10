//! `--dump-frame` `--dump-hdr-frame` `--dump-depth-frame` の書き出しの入口。担うのは「このフレームがダンプ対象か」と「どの画像をどのベース名で書き出すか」の振り分けだけであり、外部形式への写しは対象ごとの工程が持つ。
//! 親エージェントの検収工程「絵の目視監査」用(経緯: M6の影バグはvalidation・
//! ピクセル判定の両方をすり抜け、絵を見ることでのみ検出できた)。
//! 提示画像の書き出しと照合は`presentation_dump`、圧縮前のHDRの書き出しは`hdr_dump`、最終深度の書き出しは`depth_dump`が持つ。

mod cluster_assignment_check;
mod depth_dump;
mod dump_destination;
mod hdr_dump;
mod indirect_probe_check;
mod presentation_dump;
mod sky_pixel_check;

use std::path::Path;

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
        let 採るフレームか = self.現在フレーム + 1 == フレーム数
            || self.先行ダンプのフレームか(フレーム数)
            || アクション == スモークアクション::差し替え前ダンプ;
        self.フレームダンプ先.書き出すか() && 採るフレームか
    }

    /// 同一起動内の再現性を見るために、最終フレームの1つ前も書き出すか。指定が無ければ撮らない。
    pub(super) fn 先行ダンプのフレームか(&self, フレーム数: u32) -> bool {
        self.読み戻し検収.先行フレームも書き出すか && self.現在フレーム + 2 == フレーム数
    }

    pub(super) fn 読み戻してダンプする(
        &mut self,
        描画入力: blitz_render::フレーム描画入力<'_>,
        視点情報: &フレーム視点,
        アクション: スモークアクション,
    ) -> Result<描画の到達, 起動エラー> {
        match self.フレームダンプ先.clone() {
            フレームダンプ指定::指定なし => Ok(描画の到達::届かなかった),
            フレームダンプ指定::提示画像を書き出す { 基準名 } => {
                let 先 = dump_destination::決める(self, &基準名, アクション);
                presentation_dump::読み戻して書き出す(self, 描画入力, 視点情報, &先)
            }
            フレームダンプ指定::圧縮前のHDRを書き出す { 基準名 } => {
                let 先 = dump_destination::決める(self, &基準名, アクション);
                hdr_dump::読み戻して書き出す(self, 描画入力, &先)
            }
            フレームダンプ指定::最終深度を書き出す { 基準名 } => {
                let 先 = dump_destination::決める(self, &基準名, アクション);
                depth_dump::読み戻して書き出す(self, 描画入力, &先)
            }
        }
    }
}

/// 寸法ファイルは対象によらず同じ形式で書く。検収側が画素の並びを読むためにまず要るのが幅と高さであり、
/// 対象ごとに読み手を分けると同じ2数を2通りで読むことになる。
fn 寸法を書く(幅: u32, 高さ: u32, ベース名: &Path) -> Result<(), 起動エラー> {
    let sizeパス = ベース名.with_extension("size");
    std::fs::write(&sizeパス, format!("{幅} {高さ}\n"))
        .map_err(|誤り| 起動エラー::フレームダンプ失敗(format!("{}: {誤り}", sizeパス.display())))
}
