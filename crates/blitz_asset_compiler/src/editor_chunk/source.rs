//! 版付きエディターチャンクJSONの最新の形と、版の判別から最新への変換の入口。旧版・欠損・不正値を推測で補わない。
//! 版の判別は`version_dispatch`が、版ごとの型と最新への変換は`version1`から`version4`が、格子の解決は`grid_resolution`が、素材のパスの解決は`manifest_file`が、建物配置の形と検証は`placement`が、配置識別子の正準化は`placement_identifier`が、散布の群と個体の形と検証は`scatter`が、地表材質の重みの由来は`weight_source`が持つ。

mod grid_resolution;
mod manifest_file;
mod placement;
mod placement_identifier;
#[cfg(test)]
mod placement_tests;
mod scatter;
#[cfg(test)]
mod scatter_tests;
mod version1;
mod version2;
mod version3;
mod version4;
mod version_dispatch;
#[cfg(test)]
mod version_tests;
mod weight_source;

use std::path::{Path, PathBuf};

use self::manifest_file::エディターチャンクソースのファイル;
use crate::building_grid_source::{建物の格子ソース, 格子由来の建物定義};
use crate::error::アセットコンパイルエラー;
use crate::runtime_compilation::建物定義ID;
use crate::surface_material::weights::地表材質の重み格子;
pub(crate) use placement::建物配置ソース;
pub(crate) use scatter::散布の群ソース;
pub(crate) use weight_source::地表材質の重みのソース;

/// エディターが書き出すチャンクソースの最新の形式版。書き手はeditor_serverの`export/editor_chunk_source.rs`にあり、
/// 両者の版が食い違うと焼きが必ず「形式版に対応していない」で落ちる。一致は`cargo xtask conform`の定数の組が見る。
pub(super) const エディターチャンクソースの現在の形式版: u32 = 4;

pub(crate) struct エディターチャンクソース {
    高さ格子パス: PathBuf,
    地表材質の重み: 地表材質の重みのソース,
    pub(crate) 建物配置一覧: Vec<建物配置ソース>,
    pub(crate) 建物の格子一覧: Vec<格子由来の建物定義>,
    pub(crate) 散布の群一覧: Vec<散布の群ソース>,
}

impl エディターチャンクソース {
    pub(crate) fn ファイルから読む(パス: &Path) -> Result<Self, アセットコンパイルエラー> {
        let ファイル = エディターチャンクソースのファイル::生成する(パス);
        let 本文 = ファイル.本文を読む()?;
        version_dispatch::版を判別して最新の形へ変換する(&ファイル, &本文)
    }

    /// 版ごとの変換だけが呼ぶ組み立て。検証をここへ集めるため、版の側は欄の並びと写し方だけを持つ。
    fn 組み立てる(
        高さ格子パス: PathBuf,
        地表材質の重み: 地表材質の重みのソース,
        建物配置一覧: Vec<建物配置ソース>,
        建物の格子ソース一覧: Vec<建物の格子ソース>,
        散布の群一覧: Vec<散布の群ソース>,
        ファイル: &エディターチャンクソースのファイル<'_>,
    ) -> Result<Self, アセットコンパイルエラー> {
        placement::建物配置一覧を検証する(&建物配置一覧, ファイル)?;
        scatter::散布の群一覧を検証する(&散布の群一覧, ファイル)?;
        let 建物の格子一覧 = grid_resolution::格子のソース一覧を解く(&建物の格子ソース一覧, ファイル)?;
        Ok(Self {
            高さ格子パス,
            地表材質の重み,
            建物配置一覧,
            建物の格子一覧,
            散布の群一覧,
        })
    }

    /// 名指した建物定義IDの格子。持っていなければ宣言の規則から組む建物である。
    pub(crate) fn 建物の格子を引く(&self, 識別子: &建物定義ID) -> Option<&格子由来の建物定義> {
        self.建物の格子一覧.iter().find(|定義| 定義.識別子() == 識別子)
    }

    pub(crate) fn 高さ格子パス(&self) -> &Path {
        &self.高さ格子パス
    }

    /// 増分の鍵が読む、このチャンクが素材として開くファイルの一覧。
    pub(crate) fn 素材ファイルのパス一覧(&self) -> Vec<PathBuf> {
        let mut 一覧 = vec![self.高さ格子パス.clone()];
        一覧.extend(self.地表材質の重み.開くファイルのパス());
        一覧
    }

    /// 高さ格子の格子点と1対1に並ぶ重みの格子を得る。
    pub(crate) fn 地表材質の重み格子を得る(
        &self,
        高さ格子の格子点数: u32,
    ) -> Result<地表材質の重み格子, アセットコンパイルエラー> {
        self.地表材質の重み.重み格子を得る(高さ格子の格子点数)
    }
}
