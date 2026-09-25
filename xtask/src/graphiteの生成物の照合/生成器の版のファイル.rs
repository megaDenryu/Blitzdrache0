//! 入れた生成器の確かめとは、Cargo.lock と cargo が入れた道具の記録の2つのファイルの場所を保持し、それを読んで、入れた生成器が Cargo.lock の固定した
//! graphite のコミットから作られているかを確かめる係のことである。本文の読み方と比べ方は `xtask/src/graphiteの生成物の照合/生成器の版.rs` が持ち、ここはファイルを読む境界だけを持つ。
//! cargo の置き場は環境変数 `CARGO_HOME` を、無ければ利用者のホームの `.cargo` を使う(cargo 自身と同じ決め方である)。環境変数を読むのはここだけである。

use std::path::{Path, PathBuf};

use super::照合の失敗::Graphiteの生成物の照合の失敗;
use super::生成器の版::{Graphiteの固定の版, 依存の版を固定したファイルの名前, 入れた道具の記録のファイル名};

pub(super) struct 入れた生成器の確かめ {
    依存の版を固定したファイル: PathBuf,
    入れた道具の記録: PathBuf,
}

impl 入れた生成器の確かめ {
    pub(super) fn 環境から組む() -> Result<Self, Graphiteの生成物の照合の失敗> {
        let ホームのcargoの置き場 = || std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(|ホーム| Path::new(&ホーム).join(".cargo"));
        let cargoの置き場 = std::env::var_os("CARGO_HOME").map(PathBuf::from).or_else(ホームのcargoの置き場).ok_or(Graphiteの生成物の照合の失敗::cargoの置き場が分からない)?;
        Ok(Self {
            依存の版を固定したファイル: PathBuf::from(依存の版を固定したファイルの名前),
            入れた道具の記録: cargoの置き場.join(入れた道具の記録のファイル名),
        })
    }

    // Cargo.lock の固定の版と、cargo が入れた生成器の記録の出どころを照らす。記録のファイルが無いことは、生成器が1つも入っていないことと同じに扱う。
    pub(super) fn 固定の版と照らす(&self) -> Result<(), Graphiteの生成物の照合の失敗> {
        let 本文 = std::fs::read_to_string(&self.依存の版を固定したファイル).map_err(|誤り| Graphiteの生成物の照合の失敗::ファイルを読めなかった {
            パス: self.依存の版を固定したファイル.display().to_string(),
            誤り,
        })?;
        let 固定の版 = Graphiteの固定の版::依存の版を固定したファイルの本文から読む(&本文).ok_or(Graphiteの生成物の照合の失敗::固定の版が無い)?;
        let 記録 = match std::fs::read_to_string(&self.入れた道具の記録) {
            Ok(記録) => 記録,
            Err(誤り) if 誤り.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(誤り) => {
                return Err(Graphiteの生成物の照合の失敗::ファイルを読めなかった {
                    パス: self.入れた道具の記録.display().to_string(),
                    誤り,
                });
            }
        };
        固定の版.入れた生成器と照らす(&記録).map_err(|食い違い| Graphiteの生成物の照合の失敗::生成器の版が固定の版と食い違う {
            食い違い,
            入れるコマンド: 固定の版.生成器を入れるコマンド(),
        })
    }
}
