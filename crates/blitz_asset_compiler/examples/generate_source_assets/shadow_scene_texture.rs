//! シャドウ検証アセット(判断37)用の4x4白PNGの生成。床・遮蔽の両プリミティブが
//! 共有する単一のベースカラーマテリアルに使う。

use std::path::{Path, PathBuf};

const 一辺のピクセル数: u32 = 4;

/// shadow_scene_white.png(純白)を出力先へ書き出す。
pub(super) fn 保存する(出力先ディレクトリ: &Path) -> Result<(), String> {
    let パス: PathBuf = 出力先ディレクトリ.join("shadow_scene_white.png");
    let 画像 = image::RgbaImage::from_pixel(一辺のピクセル数, 一辺のピクセル数, image::Rgba([255, 255, 255, 255]));
    画像.save(&パス).map_err(|誤り| format!("{}: {誤り}", パス.display()))
}
