//! どのチャンク世界を1つの出力ルートへ焼くかの選択。1つの出力ルートは1つのカタログと1つのチャンク目録を持つため、
//! 同じ座標を持つ2つの世界は同じ出力ルートへ同居できない。世界の選択がそのまま出力ルートの選択になる。
//! どのアセットを焼くかの宣言は`asset_declaration`が、定義1件の組み立ては`definition_kind`が持ち、
//! プロセス境界の綴りとその解析は`argument_name`が持つ。宣言をコンパイラが受け取る指定へ写す手順は、
//! 小物群を`prop_group_declaration`が、目視見本を`visual_sample_declaration`が持つ。

mod argument_name;
mod asset_declaration;
mod definition_kind;
pub(super) mod prop_group_declaration;
mod vegetation_declaration;
mod vertex_diagnostic_declaration;
mod village_declaration;
pub(super) mod visual_sample_declaration;

use super::catalog::{アセット定義, ソース種別};
use vertex_diagnostic_declaration::診断の原型;

/// ソースルートからの相対で固定した、世界ごとの目録ソースの配置先。
const 板の世界の目録ソース: &str = "chunk_world/chunk_directory.txt";
const 地形の世界の目録ソース: &str = "terrain_world/chunk_directory.txt";
const 植生の世界の目録ソース: &str = "vegetation_world/chunk_directory.txt";
const 見本の集落の世界の目録ソース: &str = "village_world/chunk_directory.txt";
const 目視見本の世界の目録ソース: &str = "terrain_visual_world/chunk_directory.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 対象世界 {
    板の世界,
    地形の世界,
    植生の世界,
    見本の集落の世界,
    /// 間接照明の絵をオーナーが目で確かめるための世界。地面の上へ材質見本の立体と少数の小物を据える。
    目視見本の世界,
    /// 頂点処理量の係数を同定するための計測専用の世界。地形の代表世界と同じ地面と配置を持ち、同居植生の原型のトポロジー量だけが違う。
    頂点診断の世界(診断の原型),
}

impl 対象世界 {
    pub(super) fn 引数名から解析する(引数名: &str) -> Result<Self, String> {
        argument_name::解析する(引数名)
    }

    /// 頂点診断の世界が地形の目録を読むのは、代表世界と同じ25チャンクの同じ地面を対象にするためである。
    pub(super) fn 目録ソース相対パス(self) -> &'static str {
        match self {
            Self::板の世界 => 板の世界の目録ソース,
            Self::地形の世界 | Self::頂点診断の世界(_) => 地形の世界の目録ソース,
            Self::植生の世界 => 植生の世界の目録ソース,
            Self::見本の集落の世界 => 見本の集落の世界の目録ソース,
            Self::目視見本の世界 => 目視見本の世界の目録ソース,
        }
    }

    /// その世界のチャンクがどのソース形式で書かれているか。板はglTF、地形は高さ格子、植生は原型glTFである。
    /// 地形のチャンクだけが同居植生の宣言を伴う。本番のストリーミング経路で植生が出入りすることをこの世界で検査するためである。
    /// 同居植生の個体数だけを外から受け取るのは、物量計測が原型・マテリアル・座標を固定したまま密度を変えるためである。
    pub(super) fn チャンクのソース種別(self, 同居植生個体数: usize) -> ソース種別 {
        match self {
            Self::板の世界 => ソース種別::Gltfシーン,
            Self::地形の世界 => ソース種別::高さ格子 {
                同居植生: Some(asset_declaration::地形の同居植生(同居植生個体数)),
            },
            Self::植生の世界 => vegetation_declaration::植生種別(vegetation_declaration::計数判定の個体数),
            Self::見本の集落の世界 => ソース種別::見本の集落 {
                群一覧: village_declaration::集落の小物一覧,
            },
            Self::目視見本の世界 => ソース種別::目視見本 {
                材質見本の立体の安定id: visual_sample_declaration::材質見本の立体,
                群一覧: visual_sample_declaration::庭の小物一覧,
            },
            Self::頂点診断の世界(原型) => ソース種別::高さ格子 {
                同居植生: Some(vertex_diagnostic_declaration::同居植生(原型, 同居植生個体数)),
            },
        }
    }

    /// 同居植生の個体数を指定されなかったときに使う値。地形と頂点診断以外の世界は同居植生を持たないため、この値を読まない。
    pub(super) fn 同居植生の既定個体数() -> usize {
        asset_declaration::地形同居の既定個体数
    }

    /// チャンク以外に焼く、この世界のアセット一覧。
    pub(super) fn アセット定義一覧(self) -> Vec<アセット定義> {
        match self {
            Self::板の世界 => asset_declaration::板の世界の一覧(),
            Self::地形の世界 => asset_declaration::地形の世界の一覧(),
            Self::植生の世界 => vegetation_declaration::一覧(),
            Self::見本の集落の世界 => village_declaration::一覧(),
            Self::目視見本の世界 => visual_sample_declaration::一覧(),
            Self::頂点診断の世界(原型) => vertex_diagnostic_declaration::一覧(原型),
        }
    }
}
