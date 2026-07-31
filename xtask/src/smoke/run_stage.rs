//! blitz_appを`--frames`付きの子プロセスとして起動し、終了コードで合否を返す。
//! 起動条件の保持は`launch_setting`が担い、ここは引数列への変換と起動だけを行う。

use std::process::Command;

use super::launch_setting::起動設定;

pub(super) fn 実行する(設定: &起動設定<'_>) -> bool {
    let 引数一覧 = 引数列へ変換する(設定);
    println!("[xtask] cargo {} を実行", 引数一覧.join(" "));
    match Command::new("cargo").args(&引数一覧).status() {
        Ok(状態) => 状態.success(),
        Err(誤り) => {
            eprintln!("[xtask] cargoの起動に失敗: {誤り}");
            false
        }
    }
}

fn 引数列へ変換する(設定: &起動設定<'_>) -> Vec<String> {
    let mut 引数一覧 = vec![
        "run".to_string(),
        "-p".to_string(),
        "blitz_app".to_string(),
        "--".to_string(),
        "--frames".to_string(),
        設定.フレーム数.to_string(),
        "--shader-source".to_string(),
        設定.シェーダーパス.display().to_string(),
        "--scene".to_string(),
        設定.シーン名.to_string(),
    ];
    if let Some(root) = 設定.アセットルート {
        引数一覧.push("--asset-root".to_string());
        引数一覧.push(root.display().to_string());
    }
    if 設定.照明なし {
        引数一覧.push("--unlit".to_string());
    }
    if 設定.粒子あり {
        引数一覧.push("--particles".to_string());
    }
    if 設定.開発uiあり {
        引数一覧.push("--dev-ui".to_string());
    }
    // 厳密ピクセル判定を使うステージはポストを外し、期待値を明るさの圧縮導入前のまま保つ(判断39)。
    if 設定.ポストなし {
        引数一覧.push("--no-post".to_string());
    }
    if 設定.布あり {
        引数一覧.push("--cloth".to_string());
    }
    if 設定.ウィンドウ再構築検証あり {
        引数一覧.push("--window-rebuild".to_string());
    }
    引数一覧
}
