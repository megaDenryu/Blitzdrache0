//! 裁定待ちの語の元の一覧と短い版のファイルの置き場。2つのパスの表記をこの型の中の1箇所へ閉じ、読み書きの口を持つ。
//! パスはリポジトリの根からの相対で扱う(`cargo xtask conform` の走査と同じ前提である)。

use std::io;
use std::path::{Path, PathBuf};

use super::元の一覧::元の一覧の本文;
use super::短い版の組み立て::短い版の本文;

pub(crate) struct 裁定待ちの語のファイルの置き場 {
    元の一覧: PathBuf,
    短い版: PathBuf,
}

impl 裁定待ちの語のファイルの置き場 {
    pub(crate) fn リポジトリの根からの既定() -> Self {
        Self {
            元の一覧: PathBuf::from("_doc/計画/pending_ubiquitous_language.md"),
            短い版: PathBuf::from("_doc/計画/裁定待ちの語の短い定義.md"),
        }
    }

    pub(crate) fn 元の一覧のパス(&self) -> &Path {
        &self.元の一覧
    }

    pub(crate) fn 短い版のパス(&self) -> &Path {
        &self.短い版
    }

    pub(crate) fn 元の一覧を読む(&self) -> io::Result<元の一覧の本文> {
        Ok(元の一覧の本文::パスと原文から生成する(self.元の一覧.clone(), std::fs::read_to_string(&self.元の一覧)?))
    }

    pub(crate) fn 短い版を読む(&self) -> io::Result<短い版の本文> {
        Ok(短い版の本文::読んだ文字列から(std::fs::read_to_string(&self.短い版)?))
    }

    pub(crate) fn 短い版を書く(&self, 本文: &短い版の本文) -> io::Result<()> {
        std::fs::write(&self.短い版, 本文.文字列())
    }
}
