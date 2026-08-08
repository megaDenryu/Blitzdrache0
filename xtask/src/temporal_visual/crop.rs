//! 撮った絵から輪郭の一区画を切り取って拡大する工程。受け取るのは元の絵のパスと出力名、返すのは書き出したPNGの絶対パスである。
//!
//! 拡大に最近傍を使うのは、画素の階段そのものを見せるためである。滑らかな補間で拡大すると階段が均されて、
//! 時間再構成の有無の差が拡大の補間に埋もれる。ImageMagickを使うのは`raw_png`が既に同じ道具で読み戻しをPNGへ
//! 変換しており、道具を増やさないためである。

use std::path::{Path, PathBuf};
use std::process::Command;

/// 切り取る区画。目視見本の庭を1280x720で撮ったときの、遠景の柵と球の上端が入る位置である。
/// 柵の桟は細い斜めの線であり、画素の階段が最も強く出る対象である。
const 起点横: u32 = 560;
const 起点縦: u32 = 265;
const 区画の横: u32 = 320;
const 区画の縦: u32 = 180;
/// 拡大率。320x180を1280x720へ引き伸ばし、元の絵と同じ大きさで並べられる。
const 拡大率: u32 = 400;

/// 切り取りが絵の中に収まることを、撮った絵の寸法から確かめる。
pub(super) fn 区画が絵に収まるか確かめる(幅: usize, 高さ: usize) -> Result<(), String> {
    let 右端 = usize::try_from(起点横 + 区画の横).map_err(|誤り| format!("区画の右端を数へ直せなかった: {誤り}"))?;
    let 下端 = usize::try_from(起点縦 + 区画の縦).map_err(|誤り| format!("区画の下端を数へ直せなかった: {誤り}"))?;
    if 右端 > 幅 || 下端 > 高さ {
        return Err(format!("切り取る区画({右端}x{下端})が絵({幅}x{高さ})からはみ出している"));
    }
    Ok(())
}

pub(super) fn 切り取って拡大する(元の絵: &Path, 出力先: &Path) -> Result<PathBuf, String> {
    let 区画 = format!("{区画の横}x{区画の縦}+{起点横}+{起点縦}");
    let 状態 = Command::new("magick")
        .arg(元の絵)
        .args(["-crop", &区画, "+repage"])
        .args(["-filter", "point", "-resize", &format!("{拡大率}%")])
        .arg(出力先)
        .status()
        .map_err(|誤り| format!("ImageMagickを起動できなかった: {誤り}"))?;
    if !状態.success() {
        return Err(format!("ImageMagickが{状態}で失敗した"));
    }
    std::fs::canonicalize(出力先).map_err(|誤り| format!("{}の絶対パスを取れなかった: {誤り}", 出力先.display()))
}
