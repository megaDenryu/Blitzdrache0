//! どのチャンク世界を1つの出力ルートへ焼くかの選択。1つの出力ルートは1つのカタログと1つのチャンク目録を持つため、
//! 同じ座標を持つ2つの世界は同じ出力ルートへ同居できない。世界の選択がそのまま出力ルートの選択になる。
//! どのアセットを焼くかの宣言は`asset_declaration`が持つ。

mod asset_declaration;

use super::catalog::{アセット定義, ソース種別};

/// ソースルートからの相対で固定した、世界ごとの目録ソースの配置先。
const 板の世界の目録ソース: &str = "chunk_world/chunk_directory.txt";
const 地形の世界の目録ソース: &str = "terrain_world/chunk_directory.txt";
const 植生の世界の目録ソース: &str = "vegetation_world/chunk_directory.txt";

/// プロセス境界で世界を指す名前。xtaskが同じ綴りを渡す。
const 板の世界の引数名: &str = "chunk_world";
const 地形の世界の引数名: &str = "terrain_world";
const 植生の世界の引数名: &str = "vegetation_world";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 対象世界 {
    板の世界,
    地形の世界,
    植生の世界,
}

impl 対象世界 {
    pub(super) fn 引数名から解析する(引数名: &str) -> Result<Self, String> {
        match 引数名 {
            板の世界の引数名 => Ok(Self::板の世界),
            地形の世界の引数名 => Ok(Self::地形の世界),
            植生の世界の引数名 => Ok(Self::植生の世界),
            他 => Err(format!(
                "未知の世界名である: {他}(有効な値は{板の世界の引数名}と{地形の世界の引数名}と{植生の世界の引数名})"
            )),
        }
    }

    pub(super) fn 目録ソース相対パス(self) -> &'static str {
        match self {
            Self::板の世界 => 板の世界の目録ソース,
            Self::地形の世界 => 地形の世界の目録ソース,
            Self::植生の世界 => 植生の世界の目録ソース,
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
            Self::植生の世界 => asset_declaration::植生種別(asset_declaration::計数判定の個体数),
        }
    }

    /// 同居植生の個体数を指定されなかったときに使う値。地形以外の世界は同居植生を持たないため、この値を読まない。
    pub(super) fn 同居植生の既定個体数() -> usize {
        asset_declaration::地形同居の既定個体数
    }

    /// チャンク以外に焼く、この世界のアセット一覧。
    pub(super) fn アセット定義一覧(self) -> Vec<アセット定義> {
        match self {
            Self::板の世界 => asset_declaration::板の世界の一覧(),
            Self::地形の世界 => asset_declaration::地形の世界の一覧(),
            Self::植生の世界 => asset_declaration::植生の世界の一覧(),
        }
    }
}
