//! `--dump-frame` `--dump-hdr-frame` `--dump-depth-frame` の書き出しの入口。担うのは、検収の経路が運んできた画像の別と書き出し先を対象ごとの工程へ振り分けることだけであり、外部形式への写しは対象ごとの工程が持つ。どのフレームで書き出すかと書き出し先の名前は`検収の起動設定`が決める。
//! 親エージェントの検収工程「絵の目視監査」用(経緯: M6の影バグはvalidation・
//! ピクセル判定の両方をすり抜け、絵を見ることでのみ検出できた)。
//! 提示画像の書き出しと照合は`presentation_dump`、圧縮前のHDRの書き出しは`hdr_dump`、最終深度の書き出しは`depth_dump`が持つ。

mod cluster_assignment_check;
mod depth_dump;
mod hdr_dump;
mod indirect_probe_check;
mod presentation_dump;
mod sky_pixel_check;

use std::path::Path;

use super::draw_dispatch::描画の到達;
use super::frame::フレーム視点;
use super::screen_installation::画面の据え付け;
use super::アプリ;
use crate::cli::書き出す画像;
use crate::error::起動エラー;

impl アプリ {
    /// 検収の経路の判定がダンプを選んだ描画で呼ぶ。このフレームはスモーク判定の代わりに読み戻しとファイル書き出しを行う。
    pub(super) fn 読み戻してダンプする(&mut self, 描画入力: blitz_render::フレーム描画入力<'_>, 視点情報: &フレーム視点, 画像: 書き出す画像, 書き出し先: &Path) -> Result<描画の到達, 起動エラー> {
        let 読み戻し検収 = self.検収の起動設定.読み戻し検収();
        match 画像 {
            書き出す画像::提示画像 => {
                let 画像 = presentation_dump::提示画像を読み戻して書き出す(画面の据え付け::描画の最中に借りる(&mut self.据え付け).レンダラーを借りる(), 描画入力, 書き出し先)?;
                // 読み戻した画像を材料にする3つの照合。読み戻しの工程と触れるものが重ならないため、画像を受け取ってからここで呼ぶ。
                sky_pixel_check::空代表画素を照合する(&読み戻し検収.空の代表画素, self.天空.再現条件(), &画像, 視点情報);
                indirect_probe_check::間接照明代表板を照合する(読み戻し検収.遠方環境の検収条件, self.天空.天空状態(), &画像, 視点情報, self.大域ずらし量);
                if 読み戻し検収.クラスタ選別の割り当てを報告するか {
                    let レンダラー = 画面の据え付け::描画の最中に見る(&self.据え付け).レンダラーを見る();
                    cluster_assignment_check::クラスタ選別の割り当て統計を報告する(レンダラー, 視点情報, self.天空.ライティング());
                }
                Ok(描画の到達::提示した)
            }
            書き出す画像::圧縮前のHDR => hdr_dump::明るさ圧縮前画像を読み戻して書き出す(画面の据え付け::描画の最中に借りる(&mut self.据え付け).レンダラーを借りる(), 描画入力, 書き出し先, 読み戻し検収),
            書き出す画像::最終深度 => depth_dump::最終深度を読み戻して書き出す(画面の据え付け::描画の最中に借りる(&mut self.据え付け).レンダラーを借りる(), 描画入力, 書き出し先),
        }
    }
}

/// 寸法ファイルは対象によらず同じ形式で書く。検収側が画素の並びを読むためにまず要るのが幅と高さであり、
/// 対象ごとに読み手を分けると同じ2数を2通りで読むことになる。
fn 寸法を書く(幅: u32, 高さ: u32, ベース名: &Path) -> Result<(), 起動エラー> {
    let sizeパス = ベース名.with_extension("size");
    std::fs::write(&sizeパス, format!("{幅} {高さ}\n")).map_err(|誤り| 起動エラー::フレームダンプ失敗(format!("{}: {誤り}", sizeパス.display())))
}
