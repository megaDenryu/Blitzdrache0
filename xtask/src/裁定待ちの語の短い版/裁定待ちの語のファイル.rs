//! 裁定待ちの語の元の一覧(`裁定待ちの語の一覧のパス`)と短い版(`裁定待ちの語の短い定義のパス`)の2つのファイル。2つのパスの表記をこの型の中の1箇所へ閉じ、そのファイルを読み書きするメソッドを持つ。
//! パスはリポジトリの根からの相対で扱う(`cargo xtask conform` の走査と同じ前提である)。

use std::io;
use std::path::{Path, PathBuf};

use super::元の一覧::元の一覧の本文;
use super::短い版の組み立て::短い版の本文;

pub(crate) struct 裁定待ちの語のファイル {
    裁定待ちの語の一覧のパス: PathBuf,
    裁定待ちの語の短い定義のパス: PathBuf,
}

impl 裁定待ちの語のファイル {
    pub(crate) fn リポジトリの根からの既定() -> Self {
        Self {
            裁定待ちの語の一覧のパス: PathBuf::from("_doc/計画/pending_ubiquitous_language.md"),
            裁定待ちの語の短い定義のパス: PathBuf::from("_doc/計画/裁定待ちの語の短い定義.md"),
        }
    }

    pub(crate) fn 裁定待ちの語の一覧のパス(&self) -> &Path {
        &self.裁定待ちの語の一覧のパス
    }

    pub(crate) fn 裁定待ちの語の短い定義のパス(&self) -> &Path {
        &self.裁定待ちの語の短い定義のパス
    }

    pub(crate) fn 元の一覧を読む(&self) -> io::Result<元の一覧の本文> {
        Ok(元の一覧の本文::パスと原文から生成する(
            self.裁定待ちの語の一覧のパス.clone(),
            std::fs::read_to_string(&self.裁定待ちの語の一覧のパス)?,
        ))
    }

    pub(crate) fn 短い版を読む(&self) -> io::Result<短い版の本文> {
        Ok(短い版の本文::読んだ文字列から(std::fs::read_to_string(&self.裁定待ちの語の短い定義のパス)?))
    }

    pub(crate) fn 短い版を書く(&self, 本文: &短い版の本文) -> io::Result<()> {
        std::fs::write(&self.裁定待ちの語の短い定義のパス, 本文.文字列())
    }
}
