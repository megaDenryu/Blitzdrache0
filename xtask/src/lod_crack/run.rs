//! 1つのLOD組合せを本番アプリのストリーミング・描画経路へ渡し、最終フレームのRGBA8画像を読む工程。
//! 時間再構成は`--no-taa`で外す。この入口の判定がバイト一致に依るため、フレームをまたぐ混合が入ると前のフレームの残りが絵に混ざる。

use std::path::Path;
use std::process::Command;

use crate::acceptance::{検収の実行名, 読み戻しの書き出し先, 読み戻し画像};

use super::cases::検査条件;

pub(super) fn 描画する(出力先: &Path, 条件: &検査条件) -> Option<読み戻し画像> {
    let 実行名 = 検収の実行名::生成する(&条件.名前).map_err(|誤り| eprintln!("[xtask] {誤り}")).ok()?;
    let 書き出し先 = 読み戻しの書き出し先::出力ディレクトリの中に決める(出力先, 実行名);
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
        "--streaming-preload-radius",
        "2",
        "--streaming-ram-limit",
        "16777216",
        "--streaming-vram-limit",
        "16777216",
        "--unlit",
        "--no-post",
        // 地形世界は空を持つ方針であるため、番兵背景の露出を数えるこの検査は空パスを明示的に外す。
        "--no-sky",
        "--no-taa",
        "--dump-frame",
    ];
    let mut コマンド = Command::new("cargo");
    コマンド
        .args(引数)
        .arg(書き出し先.起動引数として渡す綴り())
        .arg("--lod-crack-pair")
        .args([
            x1.to_string(),
            z1.to_string(),
            条件.一方段.to_string(),
            x2.to_string(),
            z2.to_string(),
            条件.他方段.to_string(),
        ]);
    if let Some((欠落x, 欠落z)) = 条件.欠落 {
        コマンド.arg("--lod-crack-missing").args([欠落x.to_string(), 欠落z.to_string()]);
    }
    let 状態 = コマンド.status();
    match 状態 {
        Ok(値) if 値.success() => 読み込んで報せる(&書き出し先, &条件.名前),
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

/// 読めなかったことを条件の名前つきで報せてから無しへ畳む。この入口の呼び出し元は無しを「この組は判定しない」と読むため、
/// 読めなかった理由を捨てると、判定を飛ばしたのか合格したのかが出力から分からなくなる。
fn 読み込んで報せる(書き出し先: &読み戻しの書き出し先, 条件名: &str) -> Option<読み戻し画像> {
    match 読み戻し画像::読み込む(書き出し先) {
        Ok(画像) => Some(画像),
        Err(誤り) => {
            eprintln!("[xtask] {条件名}の読み戻しを読めなかった: {誤り}");
            None
        }
    }
}
