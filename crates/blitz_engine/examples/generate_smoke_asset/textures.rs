//! スモーク用4x4単色PNGの生成: baseColor用(純青)とホットリロード判定用(純緑、波B用)。

use std::path::{Path, PathBuf};

const 一辺のピクセル数: u32 = 4;

/// quad_base_color.png(純青)とquad_alt_color.png(純緑)を出力先へ書き出す。
pub(super) fn 保存する(出力先ディレクトリ: &Path) -> Result<(), String> {
    単色画像を保存する(
        出力先ディレクトリ.join("quad_base_color.png"),
        image::Rgba([0, 0, 255, 255]),
    )?;
    単色画像を保存する(
        出力先ディレクトリ.join("quad_alt_color.png"),
        image::Rgba([0, 255, 0, 255]),
    )?;
    Ok(())
}

fn 単色画像を保存する(パス: PathBuf, 色: image::Rgba<u8>) -> Result<(), String> {
    let 画像 = image::RgbaImage::from_pixel(一辺のピクセル数, 一辺のピクセル数, 色);
    画像
        .save(&パス)
        .map_err(|誤り| format!("{}: {誤り}", パス.display()))
}
