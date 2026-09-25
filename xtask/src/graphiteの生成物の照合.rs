//! `cargo xtask graphite-check` の入口。Graphiteの宣言を持つパッケージごとに `cargo graphite generate --check` を走らせ、
//! 生成物の本文が宣言から生成し直した結果と一致すること(手編集されていないこと)と、宣言の消えた生成物が残っていないことを確かめる(Issue #187)。
//! `cargo xtask conform` は生成物を見出しで見分けて検査から外すだけであり、本文の一致は確かめない。その穴をこの段が埋める。
//!
//! 対象のパッケージは、`crates` の直下のパッケージのうち、Cargo.toml の依存節に `graphite` の行を持つものである。
//! 対象が0件なら、照合するものが無いことを表示して成功で終わる。生成器はこのリポジトリの外の道具(`cargo install` で入れる `cargo-graphite`)であり、
//! 入っていないときと、古くて宣言を読めないとき(例: `#[derive(Clone)]` の図式)に、入れ直す手順を表示して失敗で終わる。

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use crate::conform::cargo_toml_parse::パッケージ宣言のファイル名;
use 照合の失敗::Graphiteの生成物の照合の失敗;

#[path = "graphiteの生成物の照合/照合の失敗.rs"]
mod 照合の失敗;

/// 生成器を入れる・入れ直すコマンド。生成器が無いときと照合が落ちたときの両方で表示する。
const 生成器を入れ直すコマンド: &str = "cargo install --git https://github.com/megaDenryu/Graphite graphite-cli";

pub fn 生成物を宣言から生成し直した結果と照合する() -> ExitCode {
    match Graphiteの生成物の照合::リポジトリから組む().全部のパッケージを照合する() {
        Ok(()) => ExitCode::SUCCESS,
        Err(失敗) => {
            eprintln!("[xtask] graphite-check: {失敗}");
            eprintln!("[xtask] graphite-check: 生成器が入っていないか古い場合は、`{生成器を入れ直すコマンド}` で入れ直してから再実行する");
            ExitCode::FAILURE
        }
    }
}

/// Graphiteの宣言を持つパッケージを探し、生成器で照合する係。探す起点の `crates` の置き場を保持する。
struct Graphiteの生成物の照合 {
    パッケージの置き場: PathBuf,
}

impl Graphiteの生成物の照合 {
    fn リポジトリから組む() -> Self {
        Self {
            パッケージの置き場: Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("crates"),
        }
    }

    fn 全部のパッケージを照合する(&self) -> Result<(), Graphiteの生成物の照合の失敗> {
        let パッケージ一覧 = self.graphiteに依存するパッケージ一覧()?;
        println!("[xtask] graphite-check: Graphiteに依存するパッケージ{}件を照合する", パッケージ一覧.len());
        if パッケージ一覧.is_empty() {
            return Ok(());
        }
        self.生成器が入っているかを確かめる()?;
        for パッケージ in &パッケージ一覧 {
            println!("[xtask] graphite-check: {} で cargo graphite generate --check を実行", パッケージ.display());
            let 終了状態 = Command::new("cargo")
                .args(["graphite", "generate", "--check"])
                .current_dir(パッケージ)
                .status()
                .map_err(|誤り| Graphiteの生成物の照合の失敗::コマンドを起動できなかった {
                    コマンド: "cargo graphite generate --check",
                    誤り,
                })?;
            if !終了状態.success() {
                return Err(Graphiteの生成物の照合の失敗::生成物が宣言と一致しない {
                    パッケージ: パッケージ.display().to_string(),
                    終了状態,
                });
            }
        }
        println!("[xtask] graphite-check: 成功(照合したパッケージ{}件)", パッケージ一覧.len());
        Ok(())
    }

    fn graphiteに依存するパッケージ一覧(&self) -> Result<Vec<PathBuf>, Graphiteの生成物の照合の失敗> {
        let 読めない = |誤り: std::io::Error| Graphiteの生成物の照合の失敗::パッケージの置き場を読めなかった {
            置き場: self.パッケージの置き場.display().to_string(),
            誤り,
        };
        let mut 一覧 = Vec::new();
        for 項目 in std::fs::read_dir(&self.パッケージの置き場).map_err(読めない)? {
            let パッケージ = 項目.map_err(読めない)?.path();
            let Ok(マニフェスト) = std::fs::read_to_string(パッケージ.join(パッケージ宣言のファイル名)) else {
                continue;
            };
            if マニフェスト.lines().any(graphiteの依存の行か) {
                一覧.push(パッケージ);
            }
        }
        一覧.sort();
        Ok(一覧)
    }

    // `cargo --list` が graphite のサブコマンドを挙げるかで、生成器が入っているかを確かめる。
    fn 生成器が入っているかを確かめる(&self) -> Result<(), Graphiteの生成物の照合の失敗> {
        let 出力 = Command::new("cargo")
            .arg("--list")
            .output()
            .map_err(|誤り| Graphiteの生成物の照合の失敗::コマンドを起動できなかった { コマンド: "cargo --list", 誤り })?;
        if String::from_utf8_lossy(&出力.stdout).lines().any(|行| 行.split_whitespace().next() == Some("graphite")) {
            return Ok(());
        }
        Err(Graphiteの生成物の照合の失敗::生成器が入っていない)
    }
}

// 依存節の `graphite = ...` の行か。`graphite-cli` のような名前の続く別の依存を拾わないよう、名前の直後が空白か `=` であることを求める。
fn graphiteの依存の行か(行: &str) -> bool {
    行.trim_start().strip_prefix("graphite").is_some_and(|残り| 残り.trim_start().starts_with('='))
}

#[cfg(test)]
mod tests {
    use super::graphiteの依存の行か;

    #[test]
    fn graphiteの依存の行だけを見分ける() {
        assert!(graphiteの依存の行か("graphite = { workspace = true }"));
        assert!(graphiteの依存の行か("  graphite= { git = \"https://github.com/megaDenryu/Graphite\" }"));
        assert!(!graphiteの依存の行か("graphite-cli = \"1\""));
        assert!(!graphiteの依存の行か("# graphite = 注釈"));
        assert!(!graphiteの依存の行か("thiserror = { workspace = true }"));
    }
}
