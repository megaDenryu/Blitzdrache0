//! リポジトリルート: Blitzdrache0リポジトリの直下ディレクトリを表す役割を持つパスの型。
//! `editor_web/dist`(静的配信ディレクトリ)や`assets/<世界名>/`(世界ソース出力先)のように、
//! このパスを起点に導かれる配置の綴りをこの型のメソッドへ閉じる
//! (参照: グローバルCLAUDE.md「役割の型は自分の配置を知る」)。

use std::path::{Path, PathBuf};

use blitz_asset_compiler::ソースルート;

/// リポジトリルートとは、Blitzdrache0リポジトリの直下ディレクトリのことである。
#[derive(Debug, Clone)]
pub struct リポジトリルート(PathBuf);

impl リポジトリルート {
    pub fn 生成する(パス: PathBuf) -> Self {
        Self(パス)
    }

    /// 実行時の作業ディレクトリに依存させないため、このクレート自身のマニフェストの位置
    /// (コンパイル時定数)を起点にする。`crates/editor_server/`はリポジトリ直下から
    /// 2階層下にあるため、2階層上がる。
    pub fn 解決する() -> Self {
        Self::生成する(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
    }

    pub fn パス(&self) -> &Path {
        &self.0
    }

    /// editor_webのビルド成果物を配信するディレクトリ。導出の綴りをここへ閉じる。
    pub fn 静的配信ディレクトリ(&self) -> PathBuf {
        self.0.join("editor_web/dist")
    }

    pub fn ソースルート(&self) -> ソースルート {
        ソースルート::生成する(self.0.join("assets"))
    }

    pub fn エディター実行時形式の出力先(&self) -> PathBuf {
        self.0.join("target/editor_world_assets")
    }
}
