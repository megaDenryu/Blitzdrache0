//! 1つのLOD組合せを本番アプリのストリーミング・描画経路へ渡し、最終フレームのRGBA8画像を読む工程。

use std::path::Path;
use std::process::Command;

use super::cases::検査条件;

pub(super) struct 読み戻し画像 {
    pub(super) 幅: usize,
    pub(super) 高さ: usize,
    pub(super) rgba8: Vec<u8>,
}

pub(super) fn 描画する(出力先: &Path, 条件: &検査条件) -> Option<読み戻し画像> {
    let ダンプ先 = 出力先.join(&条件.名前);
    let (x1, z1) = 条件.一方;
    let (x2, z2) = 条件.他方;
    let 引数 = [
        "run",
        "-p",
        "blitz_app",
        "--",
        "--frames",
        "120",
        "--scene",
        "terrain_origin",
        "--asset-root",
        "target/terrain_assets",
        "--streaming",
        "--streaming-ram-limit",
        "16777216",
        "--streaming-vram-limit",
        "16777216",
        "--unlit",
        "--no-post",
        "--dump-frame",
    ];
    let 状態 = Command::new("cargo")
        .args(引数)
        .arg(&ダンプ先)
        .arg("--lod-crack-pair")
        .args([
            x1.to_string(),
            z1.to_string(),
            条件.一方段.to_string(),
            x2.to_string(),
            z2.to_string(),
            条件.他方段.to_string(),
        ])
        .status();
    match 状態 {
        Ok(値) if 値.success() => 読み込む(&ダンプ先),
        Ok(値) => {
            eprintln!("[xtask] blitz_appが{}で失敗した({})", 値, 条件.名前);
            None
        }
        Err(誤り) => {
            eprintln!("[xtask] blitz_appを起動できなかった({}): {誤り}", 条件.名前);
            None
        }
    }
}

fn 読み込む(ダンプ先: &Path) -> Option<読み戻し画像> {
    let 寸法 = std::fs::read_to_string(ダンプ先.with_extension("size")).ok()?;
    let mut 要素 = 寸法.split_whitespace();
    let 幅 = 要素.next()?.parse().ok()?;
    let 高さ = 要素.next()?.parse().ok()?;
    let rgba8 = std::fs::read(ダンプ先.with_extension("raw")).ok()?;
    Some(読み戻し画像 { 幅, 高さ, rgba8 })
}
