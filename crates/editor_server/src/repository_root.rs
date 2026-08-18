//! リポジトリルートの解決。実行時の作業ディレクトリに依存させないため、
//! このクレート自身のマニフェストの位置（コンパイル時定数）を起点にする。
//! `crates/editor_server/` はリポジトリ直下から2階層下にあるため、2階層上がる。

use std::path::PathBuf;

pub fn リポジトリルートを解決する() -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
