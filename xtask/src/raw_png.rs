//! 読み戻しのRGBA8をPNGへ変換する工程。受け取るのはダンプのベース名、返すのは書き出したPNGの絶対パスである。
//! 親エージェントの検収は絵の目視を含むため、生バイト列のままでは開けない(OW3の統合DoDと同じ変換)。

use std::path::{Path, PathBuf};
use std::process::Command;

pub fn 変換する(ダンプ先: &Path) -> Result<PathBuf, String> {
    let 寸法 = std::fs::read_to_string(ダンプ先.with_extension("size")).map_err(|誤り| format!("読み戻し寸法を読めなかった: {誤り}"))?;
    let 大きさ = 寸法.split_whitespace().collect::<Vec<_>>().join("x");
    let raw = ダンプ先.with_extension("raw");
    let png = ダンプ先.with_extension("png");
    let 状態 = Command::new("magick")
        .args(["-size", &大きさ, "-depth", "8"])
        .arg(format!("rgba:{}", raw.display()))
        .arg(&png)
        .status()
        .map_err(|誤り| format!("ImageMagickを起動できなかった: {誤り}"))?;
    if !状態.success() {
        return Err(format!("ImageMagickが{状態}で失敗した"));
    }
    std::fs::canonicalize(&png).map_err(|誤り| format!("{}の絶対パスを取れなかった: {誤り}", png.display()))
}
