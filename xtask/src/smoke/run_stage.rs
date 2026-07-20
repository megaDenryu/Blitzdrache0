//! blitz_appを`--frames`付きの子プロセスとして起動し、終了コードで合否を返す。

use std::path::Path;
use std::process::Command;

#[allow(clippy::too_many_arguments)]
pub(super) fn 実行する(
    フレーム数: &str,
    シェーダーパス: &Path,
    アセットルート: Option<&Path>,
    シーン名: &str,
    unlit: bool,
    particles: bool,
    dev_ui: bool,
) -> bool {
    let mut 引数一覧 = vec![
        "run".to_string(),
        "-p".to_string(),
        "blitz_app".to_string(),
        "--".to_string(),
        "--frames".to_string(),
        フレーム数.to_string(),
        "--shader-source".to_string(),
        シェーダーパス.display().to_string(),
        "--scene".to_string(),
        シーン名.to_string(),
    ];
    if let Some(root) = アセットルート {
        引数一覧.push("--asset-root".to_string());
        引数一覧.push(root.display().to_string());
    }
    if unlit {
        引数一覧.push("--unlit".to_string());
    }
    if particles {
        引数一覧.push("--particles".to_string());
    }
    if dev_ui {
        引数一覧.push("--dev-ui".to_string());
    }

    println!("[xtask] cargo {} を実行", 引数一覧.join(" "));
    match Command::new("cargo").args(&引数一覧).status() {
        Ok(状態) => 状態.success(),
        Err(誤り) => {
            eprintln!("[xtask] cargoの起動に失敗: {誤り}");
            false
        }
    }
}
