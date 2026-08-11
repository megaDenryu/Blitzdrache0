//! 検査するファイル一覧: glTFの入力契約を検査させるファイルの並び。
//!
//! 1件以上あることを構築の口で課すのは、空の一覧で検査器を起こすと何も検査せずに成功が返るためである。
//! 引数の並びを組む段でこれを見ていたときは、空で起こす形が型としては成立していた。ここで課せば構築できない。

use std::path::{Path, PathBuf};

use super::error::生成器エラー;

/// 注意: 1件以上を持つことがこの型の不変条件である。構築の口は下の1つだけである。
#[repr(transparent)]
pub struct 検査するファイル一覧(Vec<PathBuf>);

impl 検査するファイル一覧 {
    /// コマンド行で受け取った語の並びから作る。1件も無い並びは型付きエラーで返す。
    pub fn コマンド行の語から生成する(語一覧: &[String]) -> Result<Self, 生成器エラー> {
        if 語一覧.is_empty() {
            return Err(生成器エラー::検査するファイルが1つも無い);
        }
        Ok(Self(語一覧.iter().map(PathBuf::from).collect()))
    }

    pub(super) fn 一覧(&self) -> impl Iterator<Item = &Path> {
        self.0.iter().map(PathBuf::as_path)
    }
}
