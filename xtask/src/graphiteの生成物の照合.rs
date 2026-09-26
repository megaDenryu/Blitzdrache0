//! `cargo xtask graphite-check` の入口。Graphiteの宣言を持つパッケージごとに `cargo graphite generate --check` を走らせ、
//! 生成物の本文が宣言から生成し直した結果と一致すること(手編集されていないこと)と、宣言の消えた生成物が残っていないことを確かめる(Issue #187)。
//! `cargo xtask conform` は生成物を見出しで見分けて検査から外すだけであり、本文の一致は確かめない。その穴をこの段が埋める。
//!
//! 照合の前に2つを確かめる。1つは、conform が生成物と認めたファイルを持つパッケージが、照合の対象(依存の節に graphite を持つ `crates` の直下のパッケージ)に
//! 全部入っていること(`xtask/src/graphiteの生成物の照合/生成物を持つパッケージ.rs`)。もう1つは、入れた生成器が Cargo.lock の固定した graphite のコミットから
//! 作られていること(`xtask/src/graphiteの生成物の照合/生成器の版のファイル.rs`)。生成器はこのリポジトリの外の道具(`cargo install` で入れる `cargo-graphite`)であり、
//! 入っていないとき・別のコミットから入れたとき・パスから入れたときは、Cargo.lock の版を `--rev` に含む入れ直しのコマンドを表示して失敗で終わる。
//! 対象が0件で生成物も無いなら、照合するものが無いことを表示して成功で終わる。パスはリポジトリの根からの相対で扱う(conform の走査と同じ前提である)。

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use crate::conform::cargo_toml_parse::パッケージ宣言のファイル名;
use crate::conform::走査した原文の一覧::走査した原文の一覧;
use crate::type_metrics::走査対象の原文を集める;
use graphiteへの依存::graphiteに依存するか;
use 照合の失敗::Graphiteの生成物の照合の失敗;
use 生成器の版のファイル::入れた生成器の確かめ;
use 生成物を持つパッケージ::生成物を持つパッケージの一覧;

#[path = "graphiteの生成物の照合/graphiteへの依存.rs"]
mod graphiteへの依存;
#[path = "graphiteの生成物の照合/照合の失敗.rs"]
mod 照合の失敗;
#[path = "graphiteの生成物の照合/生成器の版.rs"]
mod 生成器の版;
#[path = "graphiteの生成物の照合/生成器の版のファイル.rs"]
mod 生成器の版のファイル;
#[path = "graphiteの生成物の照合/生成物を持つパッケージ.rs"]
mod 生成物を持つパッケージ;

pub fn 生成物を宣言から生成し直した結果と照合する() -> ExitCode {
    match Graphiteの生成物の照合::リポジトリから組む().and_then(|照合| 照合.全部のパッケージを照合する()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(失敗) => {
            eprintln!("[xtask] graphite-check: {失敗}");
            ExitCode::FAILURE
        }
    }
}

/// Graphiteの宣言を持つパッケージを探し、生成器で照合する係。探す起点の `crates` の置き場と、入れた生成器の版の確かめを保持する。
struct Graphiteの生成物の照合 {
    パッケージの置き場: PathBuf,
    入れた生成器: 入れた生成器の確かめ,
}

impl Graphiteの生成物の照合 {
    fn リポジトリから組む() -> Result<Self, Graphiteの生成物の照合の失敗> {
        Ok(Self {
            パッケージの置き場: PathBuf::from("crates"),
            入れた生成器: 入れた生成器の確かめ::環境から組む()?,
        })
    }

    fn 全部のパッケージを照合する(&self) -> Result<(), Graphiteの生成物の照合の失敗> {
        let パッケージ一覧 = self.graphiteに依存するパッケージ一覧()?;
        let 原文一覧 = 走査対象の原文を集める().map_err(|破れ| Graphiteの生成物の照合の失敗::原文を読めなかった { 破れ: 破れ.to_string() })?;
        let 生成物を持つ = 生成物を持つパッケージの一覧::原文一覧から組む(&走査した原文の一覧::生成する(原文一覧));
        println!(
            "[xtask] graphite-check: Graphiteに依存するパッケージ{}件を照合する(Graphiteの生成物を持つパッケージは{}件)",
            パッケージ一覧.len(),
            生成物を持つ.件数()
        );
        let 対象に無い = 生成物を持つ.照合の対象に無いパッケージ一覧(&パッケージ一覧);
        if !対象に無い.is_empty() {
            return Err(Graphiteの生成物の照合の失敗::生成物を持つパッケージが照合の対象に無い { パッケージ一覧: 対象に無い });
        }
        if パッケージ一覧.is_empty() {
            return Ok(());
        }
        self.入れた生成器.固定の版と照らす()?;
        for パッケージ in &パッケージ一覧 {
            self.パッケージを1つ照合する(パッケージ)?;
        }
        println!("[xtask] graphite-check: 成功(照合したパッケージ{}件)", パッケージ一覧.len());
        Ok(())
    }

    fn graphiteに依存するパッケージ一覧(&self) -> Result<Vec<PathBuf>, Graphiteの生成物の照合の失敗> {
        let 読めない = |誤り: std::io::Error| Graphiteの生成物の照合の失敗::ファイルを読めなかった {
            パス: self.パッケージの置き場.display().to_string(),
            誤り,
        };
        let mut 一覧 = Vec::new();
        for 項目 in std::fs::read_dir(&self.パッケージの置き場).map_err(読めない)? {
            let パッケージ = 項目.map_err(読めない)?.path();
            let Ok(マニフェスト) = std::fs::read_to_string(パッケージ.join(パッケージ宣言のファイル名)) else {
                continue;
            };
            if graphiteに依存するか(&マニフェスト) {
                一覧.push(パッケージ);
            }
        }
        一覧.sort();
        Ok(一覧)
    }

    // パッケージの置き場で生成器の照合を1回走らせる。
    fn パッケージを1つ照合する(&self, パッケージ: &Path) -> Result<(), Graphiteの生成物の照合の失敗> {
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
        Ok(())
    }
}
