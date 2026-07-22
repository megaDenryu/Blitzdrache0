//! blitz_appを`--frames`付きの子プロセスとして起動し、終了コードで合否を返す。

use std::path::Path;
use std::process::Command;

pub(super) struct 起動設定<'a> {
    フレーム数: &'a str,
    シェーダーパス: &'a Path,
    アセットルート: Option<&'a Path>,
    シーン名: &'a str,
    照明なし: bool,
    粒子あり: bool,
    開発uiあり: bool,
    ポストなし: bool,
    布あり: bool,
}

impl<'a> 起動設定<'a> {
    pub(super) fn 生成する(
        フレーム数: &'a str, シェーダーパス: &'a Path, アセットルート: Option<&'a Path>, シーン名: &'a str
    ) -> Self {
        Self {
            フレーム数,
            シェーダーパス,
            アセットルート,
            シーン名,
            照明なし: false,
            粒子あり: false,
            開発uiあり: false,
            ポストなし: false,
            布あり: false,
        }
    }

    pub(super) fn 照明なし(mut self) -> Self {
        self.照明なし = true;
        self
    }
    pub(super) fn 粒子あり(mut self) -> Self {
        self.粒子あり = true;
        self
    }
    pub(super) fn 開発uiあり(mut self) -> Self {
        self.開発uiあり = true;
        self
    }
    pub(super) fn ポストなし(mut self) -> Self {
        self.ポストなし = true;
        self
    }
    pub(super) fn 布あり(mut self) -> Self {
        self.布あり = true;
        self
    }
}

pub(super) fn 実行する(設定: &起動設定<'_>) -> bool {
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
    // 厳密ピクセル判定を使うステージはポストを外し、期待値をトーンマップ導入前のまま保つ(判断39)。
    if 設定.ポストなし {
        引数一覧.push("--no-post".to_string());
    }
    if 設定.布あり {
        引数一覧.push("--cloth".to_string());
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
