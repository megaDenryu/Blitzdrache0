//! 検証世界のソースの書き出しの工程。受け取るのはリポジトリルートと建物定義の識別子であり、返すのは書き出せたかどうかである。
//! 書き出し先(`assets/one_building_world/`)は編集サーバーの側が自分の配置として知っているため、ここからは渡さない。
//!
//! 編集サーバーのビンを子プロセスとして起こすのは、xtaskが依存の白リストで`editor_server`へ依存できないためである
//! (契約の書き出しが`contract_export`のビンを起こすのと同じ形である)。

use std::path::Path;
use std::process::Command;

const ビン名: &str = "bake_one_building";

pub(super) fn 編集サーバーのビンで書き出す(
    リポジトリルート: &Path, 建物定義の識別子: &str, 引数一覧: &[String]
) -> Result<(), String> {
    let 終了状態 = Command::new("cargo")
        .args(["run", "-p", "editor_server", "--bin", ビン名, "--"])
        .arg(建物定義の識別子)
        .args(プロジェクトの指定を引き継ぐ(引数一覧))
        .current_dir(リポジトリルート)
        .status()
        .map_err(|原因| format!("{ビン名}の起動に失敗した: {原因}"))?;
    if 終了状態.success() {
        return Ok(());
    }
    Err(format!("{ビン名}が建物{建物定義の識別子}のソースを書き出せなかった"))
}

/// `--project <ルート>`が付いていればそのままビンへ渡す。検証用の使い捨てのプロジェクトを開いた実行が、
/// リポジトリ側の格子で焼いてしまうのを防ぐためである。
fn プロジェクトの指定を引き継ぐ(引数一覧: &[String]) -> Vec<String> {
    引数一覧
        .iter()
        .position(|引数| 引数 == "--project")
        .and_then(|添字| 引数一覧.get(添字 + 1))
        .map_or_else(Vec::new, |ルート| vec!["--project".to_string(), ルート.clone()])
}
