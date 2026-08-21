//! 版付きエディターチャンクJSONの読み取りと検証。旧版・欠損・不正値を推測で補わない。

use std::{
    collections::HashSet,
    path::{Component, Path, PathBuf},
};

use serde::Deserialize;

use crate::error::アセットコンパイルエラー;
use crate::runtime_compilation::建物定義ID;

const 現在の形式版: u32 = 1;

#[derive(Debug, Deserialize)]
pub(crate) struct エディターチャンクソース {
    形式版: u32,
    高さ格子: String,
    pub(crate) 建物配置一覧: Vec<建物配置ソース>,
    #[serde(skip)]
    高さ格子パス: PathBuf,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct 建物配置ソース {
    pub(crate) 配置識別子: String,
    pub(crate) 建物定義ID: 建物定義ID,
    pub(crate) チャンク原点からの東メートル: f32,
    pub(crate) チャンク原点からの南メートル: f32,
    pub(crate) 向きラジアン: f32,
}

impl エディターチャンクソース {
    pub(crate) fn ファイルから読む(パス: &Path) -> Result<Self, アセットコンパイルエラー> {
        let 本文 = std::fs::read_to_string(パス).map_err(|原因| 読み込み失敗(パス, 原因.to_string()))?;
        let mut ソース: Self = serde_json::from_str(&本文).map_err(|原因| 読み込み失敗(パス, format!("JSONが不正である: {原因}")))?;
        if ソース.形式版 != 現在の形式版 {
            return Err(読み込み失敗(
                パス,
                format!("形式版{}には対応していない（対応版: {現在の形式版}）", ソース.形式版),
            ));
        }
        let 相対 = Path::new(&ソース.高さ格子);
        if 相対.is_absolute() || 相対.components().any(|成分| !matches!(成分, Component::Normal(_))) {
            return Err(読み込み失敗(パス, format!("高さ格子の相対パスが不正である: {}", ソース.高さ格子)));
        }
        let 親 = パス.parent().ok_or_else(|| 読み込み失敗(パス, "親ディレクトリが無い".to_string()))?;
        ソース.高さ格子パス = 親.join(相対);
        let mut 配置識別子一覧 = HashSet::new();
        for 配置 in &ソース.建物配置一覧 {
            if 配置.配置識別子.trim().is_empty()
                || !配置識別子一覧.insert(&配置.配置識別子)
                || !配置.チャンク原点からの東メートル.is_finite()
                || !配置.チャンク原点からの南メートル.is_finite()
                || !配置.向きラジアン.is_finite()
            {
                return Err(読み込み失敗(パス, "建物配置に空の識別子または有限でない数値がある".to_string()));
            }
        }
        Ok(ソース)
    }

    pub(crate) fn 高さ格子パス(&self) -> &Path {
        &self.高さ格子パス
    }
}

fn 読み込み失敗(パス: &Path, 原因: String) -> アセットコンパイルエラー {
    アセットコンパイルエラー::エディターチャンクソース不正 {
        パス: パス.display().to_string(),
        原因,
    }
}
