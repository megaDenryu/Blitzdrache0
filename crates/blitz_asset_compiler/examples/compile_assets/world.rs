//! どのチャンク世界を1つの出力ルートへ焼くかの選択。1つの出力ルートは1つのカタログと1つのチャンク目録を持つため、
//! 同じ座標を持つ2つの世界は同じ出力ルートへ同居できない。世界の選択がそのまま出力ルートの選択になる。

use super::catalog::ソース種別;

/// ソースルートからの相対で固定した、世界ごとの目録ソースの配置先。
const 板の世界の目録ソース: &str = "chunk_world/chunk_directory.txt";
const 地形の世界の目録ソース: &str = "terrain_world/chunk_directory.txt";

/// プロセス境界で世界を指す名前。xtaskが同じ綴りを渡す。
const 板の世界の引数名: &str = "chunk_world";
const 地形の世界の引数名: &str = "terrain_world";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 対象世界 {
    板の世界,
    地形の世界,
}

impl 対象世界 {
    pub(super) fn 引数名から解析する(引数名: &str) -> Result<Self, String> {
        match 引数名 {
            板の世界の引数名 => Ok(Self::板の世界),
            地形の世界の引数名 => Ok(Self::地形の世界),
            他 => Err(format!("未知の世界名である: {他}(有効な値は{板の世界の引数名}と{地形の世界の引数名})")),
        }
    }

    pub(super) fn 目録ソース相対パス(self) -> &'static str {
        match self {
            Self::板の世界 => 板の世界の目録ソース,
            Self::地形の世界 => 地形の世界の目録ソース,
        }
    }

    /// その世界のチャンクがどのソース形式で書かれているか。板はglTF、地形は高さ格子である。
    pub(super) fn チャンクのソース種別(self) -> ソース種別 {
        match self {
            Self::板の世界 => ソース種別::Gltfシーン,
            Self::地形の世界 => ソース種別::高さ格子,
        }
    }
}
