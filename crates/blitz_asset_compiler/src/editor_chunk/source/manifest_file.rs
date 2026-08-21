//! 版付きエディターチャンクJSONのファイルという役割を持つパスの型。担当するのは、本文の読み取りと、
//! そのファイルの隣に置かれた素材への相対パスの解決と、この1つのファイルに紐づく失敗の組み立てである。
//!
//! 相対パスの解決をこの型のメソッドへ閉じるのは、絶対パスと親ディレクトリへ遡る成分を弾く判定を、
//! 高さ格子と地表材質の重み格子の2箇所へ書き分けないためである。

use std::path::{Component, Path, PathBuf};

use serde::Deserialize;

use crate::error::アセットコンパイルエラー;

pub(super) struct エディターチャンクソースのファイル<'パス> {
    パス: &'パス Path,
}

impl<'パス> エディターチャンクソースのファイル<'パス> {
    pub(super) fn 生成する(パス: &'パス Path) -> Self {
        Self { パス }
    }

    pub(super) fn 本文を読む(&self) -> Result<String, アセットコンパイルエラー> {
        std::fs::read_to_string(self.パス).map_err(|原因| self.読み込み失敗のエラーを作る(原因.to_string()))
    }

    /// 本文を版の型として解析する。失敗の綴りをこの1箇所へ集めるため、版ごとに書き分けない。
    pub(super) fn 版の型として解析する<版の型: for<'本文> Deserialize<'本文>>(
        &self,
        本文: &str,
    ) -> Result<版の型, アセットコンパイルエラー> {
        serde_json::from_str(本文).map_err(|原因| self.読み込み失敗のエラーを作る(format!("JSONが不正である: {原因}")))
    }

    /// このファイルと同じディレクトリの下にある素材のパスを組む。絶対パスと親へ遡る成分は、
    /// ソースディレクトリの外を指す経路になるため型付きの失敗にする。
    pub(super) fn 直下の相対パスを解決する(&self, 相対: &str) -> Result<PathBuf, アセットコンパイルエラー> {
        let 相対パス = Path::new(相対);
        if 相対パス.is_absolute() || 相対パス.components().any(|成分| !matches!(成分, Component::Normal(_))) {
            return Err(self.読み込み失敗のエラーを作る(format!("素材の相対パスが不正である: {相対}")));
        }
        let 親 = self
            .パス
            .parent()
            .ok_or_else(|| self.読み込み失敗のエラーを作る("親ディレクトリが無い".to_string()))?;
        Ok(親.join(相対パス))
    }

    pub(super) fn 読み込み失敗のエラーを作る(&self, 原因: String) -> アセットコンパイルエラー {
        アセットコンパイルエラー::エディターチャンクソース不正 {
            パス: self.パス.display().to_string(),
            原因,
        }
    }
}
