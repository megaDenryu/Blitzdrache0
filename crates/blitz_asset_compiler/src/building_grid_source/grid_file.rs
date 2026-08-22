//! 建物の格子1件のファイル。配置と読み書きと形式版の判別をこの型へ閉じる。
//!
//! 版を先に読んでから版の型として解くのは、欄の並びが違う旧版のファイルを最新の型で解こうとして
//! 「欄が無い」という読みにくい失敗になるのを避けるためである。形は`editor_chunk/source`に倣う。
//!
//! ファイル名の綴りを置き場(`directory`)でなくここが持たないのは、置き場が建物ごとのディレクトリを
//! 数え上げる責務も持ち、綴りが2つの導出に要るためである。綴りの正本は置き場にある。

use std::path::{Path, PathBuf};

use super::error::建物の格子のソースエラー;
use super::grid_definition::格子由来の建物定義;
use super::source::{建物の格子ソース, 建物の格子ソースの現在の形式版};

/// 形式版だけを先に読むための最小の形。
#[derive(serde::Deserialize)]
struct 形式版の名乗り {
    形式版: u32,
}

pub struct 建物の格子のファイル {
    パス: PathBuf,
}

impl 建物の格子のファイル {
    pub(super) fn 生成する(パス: PathBuf) -> Self {
        Self { パス }
    }

    pub fn パス(&self) -> &Path {
        &self.パス
    }

    /// 版を判別してから最新の型として解き、ベイ格子まで通す。
    pub fn 読んで解く(&self) -> Result<格子由来の建物定義, 建物の格子のソースエラー> {
        let ソース = self.読んで最新の形にする()?;
        ソース.格子由来の建物定義へ解く()
    }

    pub fn 読んで最新の形にする(&self) -> Result<建物の格子ソース, 建物の格子のソースエラー> {
        let 本文 = std::fs::read_to_string(&self.パス).map_err(|原因| 建物の格子のソースエラー::ファイルを読めない {
            パス: self.パスの綴り(),
            原因,
        })?;
        let 名乗り: 形式版の名乗り = serde_json::from_str(&本文).map_err(|原因| self.解釈できないエラーを作る(原因))?;
        match 名乗り.形式版 {
            1 => serde_json::from_str(&本文).map_err(|原因| self.解釈できないエラーを作る(原因)),
            未対応 => Err(建物の格子のソースエラー::形式版に対応していない {
                パス: self.パスの綴り(),
                版: 未対応,
                対応上限: 建物の格子ソースの現在の形式版,
            }),
        }
    }

    /// 一時ファイル経由で書き込む。整形して改行で終える形は建物外形カタログと同じである。
    pub fn 書き出す(&self, ソース: &建物の格子ソース) -> Result<(), 建物の格子のソースエラー> {
        let 建物定義 = ソース.建物定義ID.綴り().to_string();
        let mut バイト列 =
            serde_json::to_vec_pretty(ソース).map_err(|原因| 建物の格子のソースエラー::Jsonを組み立てられない {
                建物定義: 建物定義.clone(),
                原因,
            })?;
        バイト列.push(b'\n');
        if let Some(親) = self.パス.parent() {
            std::fs::create_dir_all(親).map_err(|原因| self.書き出せないエラーを作る(&建物定義, 原因))?;
        }
        crate::atomic_file_write::一時ファイル経由で書き込む(&self.パス, &バイト列)
            .map_err(|原因| self.書き出せないエラーを作る(&建物定義, 原因))
    }

    fn パスの綴り(&self) -> String {
        self.パス.display().to_string()
    }

    fn 解釈できないエラーを作る(&self, 原因: serde_json::Error) -> 建物の格子のソースエラー {
        建物の格子のソースエラー::Jsonを解釈できない {
            パス: self.パスの綴り(),
            原因,
        }
    }

    fn 書き出せないエラーを作る(&self, 建物定義: &str, 原因: std::io::Error) -> 建物の格子のソースエラー {
        建物の格子のソースエラー::書き出せない {
            建物定義: 建物定義.to_string(),
            原因,
        }
    }
}
