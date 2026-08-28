//! 版付きエディターチャンクソースが運ぶ建物配置1件の型と、配置一覧の検証。担当するのは、
//! 建物配置の直列化の形と、識別子の重複・有限でない数値の2つを拒む判定である。
//!
//! 識別子の空を判定しないのは、`配置識別子`の読み取りが正準化と空の拒否を済ませているためである。
//! 重複を正準化した綴りで判定するのは、下流の種・描画・物理が同じ綴りで同じ配置を指すためである。
//!
//! 形式版1と形式版2が同じ形で建物配置を持つため、版ごとの型はこの1つを共有する。

use std::collections::HashSet;

use serde::Deserialize;

use super::manifest_file::エディターチャンクソースのファイル;
use super::placement_identifier::配置識別子;
use crate::error::アセットコンパイルエラー;
use crate::runtime_compilation::建物定義ID;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct 建物配置ソース {
    pub(crate) 配置識別子: 配置識別子,
    pub(crate) 建物定義ID: 建物定義ID,
    pub(crate) チャンク原点からの東メートル: f32,
    pub(crate) チャンク原点からの南メートル: f32,
    pub(crate) 向きラジアン: f32,
}

pub(super) fn 建物配置一覧を検証する(
    一覧: &[建物配置ソース],
    ファイル: &エディターチャンクソースのファイル<'_>,
) -> Result<(), アセットコンパイルエラー> {
    let mut 出会った配置識別子 = HashSet::new();
    for 配置 in 一覧 {
        if !出会った配置識別子.insert(配置.配置識別子.綴り()) {
            return Err(ファイル.読み込み失敗のエラーを作る(format!("建物配置の識別子{}が重複している", 配置.配置識別子)));
        }
        if !配置.チャンク原点からの東メートル.is_finite() || !配置.チャンク原点からの南メートル.is_finite() || !配置.向きラジアン.is_finite()
        {
            return Err(ファイル.読み込み失敗のエラーを作る(format!("建物配置{}に有限でない数値がある", 配置.配置識別子)));
        }
    }
    Ok(())
}
