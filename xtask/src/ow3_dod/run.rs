//! 半径2で25チャンクを先読みしてから100m境界とLOD閾値を横切る本番アプリ実行と、RGBA8からPNGへの変換を担う。

use std::path::{Path, PathBuf};
use std::process::Command;

pub(super) struct 実行結果 {
    pub(super) 標準出力: String,
    pub(super) 標準エラー: String,
    pub(super) ダンプ先: PathBuf,
}

pub(super) fn 統合経路を実行する() -> Option<実行結果> {
    let ダンプ先 = PathBuf::from("target/ow3_dod/multi_lod");
    std::fs::create_dir_all(ダンプ先.parent()?).ok()?;
    let 出力 = Command::new("cargo")
        .args([
            "run",
            "-p",
            "blitz_app",
            "--",
            "--frames",
            "160",
            "--scene",
            "terrain_origin",
            "--asset-root",
            "target/terrain_assets",
            "--streaming",
            "--streaming-preload-radius",
            "2",
            "--streaming-ram-limit",
            "16777216",
            "--streaming-vram-limit",
            "16777216",
            "--ow3-dod-route",
            "--report-streaming-summary",
            "--report-memory",
            "--dump-frame",
        ])
        .arg(&ダンプ先)
        .output()
        .ok()?;
    if !出力.status.success() {
        eprintln!("[xtask] 統合経路が{}で失敗した", 出力.status);
        return None;
    }
    Some(実行結果 {
        標準出力: String::from_utf8(出力.stdout).ok()?,
        標準エラー: String::from_utf8(出力.stderr).ok()?,
        ダンプ先,
    })
}

pub(super) fn pngへ変換する(ダンプ先: &Path) -> Option<PathBuf> {
    let 寸法 = std::fs::read_to_string(ダンプ先.with_extension("size")).ok()?;
    let 大きさ = 寸法.split_whitespace().collect::<Vec<_>>().join("x");
    let raw = ダンプ先.with_extension("raw");
    let png = ダンプ先.with_extension("png");
    let 状態 = Command::new("magick")
        .args(["-size", &大きさ, "-depth", "8"])
        .arg(format!("rgba:{}", raw.display()))
        .arg(&png)
        .status()
        .ok()?;
    if !状態.success() {
        eprintln!("[xtask] ImageMagickが{状態}で失敗した");
        return None;
    }
    std::fs::canonicalize(png).ok()
}
