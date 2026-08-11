//! 読み戻しのRGBA8をPNGへ変換する工程。受け取るのはダンプのベース名、返すのは書き出したPNGの絶対パスである。
//! 親エージェントの検収は絵の目視を含むため、生バイト列のままでは開けない(OW3の統合DoDと同じ変換)。
//! 破れうる前提の数え上げは`error`が持つ。

mod error;

use std::path::{Path, PathBuf};
use std::process::Command;

pub use error::目視用の絵への変換の破れ;

pub fn 変換する(ダンプ先: &Path) -> Result<PathBuf, 目視用の絵への変換の破れ> {
    let 寸法のパス = ダンプ先.with_extension("size");
    let 寸法 = std::fs::read_to_string(&寸法のパス)
        .map_err(|誤り| 目視用の絵への変換の破れ::読み戻し寸法を読めなかった { 寸法のパス, 誤り })?;
    let 大きさ = 寸法.split_whitespace().collect::<Vec<_>>().join("x");
    let raw = ダンプ先.with_extension("raw");
    let png = ダンプ先.with_extension("png");
    let 状態 = Command::new("magick")
        .args(["-size", &大きさ, "-depth", "8"])
        .arg(format!("rgba:{}", raw.display()))
        .arg(&png)
        .status()
        .map_err(|誤り| 目視用の絵への変換の破れ::変換ツールを起こせなかった { 誤り })?;
    if !状態.success() {
        return Err(目視用の絵への変換の破れ::変換ツールが失敗して終わった {
            終了状態: 状態.to_string()
        });
    }
    std::fs::canonicalize(&png)
        .map_err(|誤り| 目視用の絵への変換の破れ::絵の絶対パスを取れなかった { 絵のパス: png, 誤り })
}
