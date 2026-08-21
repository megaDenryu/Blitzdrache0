//! 版付きエディターチャンクソースが運ぶ建物配置1件の型と、配置一覧の検証。担当するのは、
//! 建物配置の直列化の形と、識別子の空・重複・有限でない数値の3つを拒む判定である。
//!
//! 形式版1と形式版2が同じ形で建物配置を持つため、版ごとの型はこの1つを共有する。

use std::collections::HashSet;

use serde::Deserialize;

use super::manifest_file::エディターチャンクソースのファイル;
use crate::error::アセットコンパイルエラー;
use crate::runtime_compilation::建物定義ID;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct 建物配置ソース {
    pub(crate) 配置識別子: String,
    pub(crate) 建物定義ID: 建物定義ID,
    pub(crate) チャンク原点からの東メートル: f32,
    pub(crate) チャンク原点からの南メートル: f32,
    pub(crate) 向きラジアン: f32,
}

pub(super) fn 建物配置一覧を検証する(
    一覧: &[建物配置ソース],
    ファイル: &エディターチャンクソースのファイル<'_>,
) -> Result<(), アセットコンパイルエラー> {
    let mut 配置識別子一覧 = HashSet::new();
    for 配置 in 一覧 {
        if 配置.配置識別子.trim().is_empty()
            || !配置識別子一覧.insert(&配置.配置識別子)
            || !配置.チャンク原点からの東メートル.is_finite()
            || !配置.チャンク原点からの南メートル.is_finite()
            || !配置.向きラジアン.is_finite()
        {
            return Err(ファイル.読み込み失敗のエラーを作る("建物配置に空の識別子または有限でない数値がある".to_string()));
        }
    }
    Ok(())
}
