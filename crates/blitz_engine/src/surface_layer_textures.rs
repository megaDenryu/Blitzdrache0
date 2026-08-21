//! 地表層テクスチャ集: 地表材質の層に貼るタイルテクスチャを、世界に1つだけ置く共有アセットとして持つ語彙。
//! 高さ場と同じく、固定の安定IDでカタログから引く単一のアセットであり、チャンク目録には載らない。
//!
//! 材質名で並べたタイルと、層から材質へ結ぶ索引を分けて持つのは、エディターのマテリアル台帳が
//! 「層はどの材質名を参照するか」という形で塗り分けを決めるためである。名前から索引への解決は
//! 焼く側が行い、実行時に名前で引く経路は持たない。
//! 参照: `_doc/設計/マルチマテリアルと材質境界.md`

mod error;
mod layer_index;
mod load_error;
mod reader;
mod stable_id;
mod texture_set;
mod tile;

pub use error::地表層テクスチャ集エラー;
pub use layer_index::地表層ごとの材質索引;
pub use load_error::地表層テクスチャ集読込エラー;
pub use reader::地表層テクスチャ集の読み口;
pub use stable_id::世界の地表層テクスチャ集の安定IDの綴り;
pub use texture_set::地表層テクスチャ集;
pub use tile::地表層のタイル;

/// 実行時形式との往復は`asset/runtime_format/surface_layer_texture_set_v1`が実装し、その入口をここから見せ直す。
pub use crate::asset::runtime_format::{
    地表層テクスチャ集を実行時形式へ格納する, 地表層テクスチャ集実行時形式エラー, 実行時形式から地表層テクスチャ集を読む,
};

/// 塗り分けの層の数。草・泥・岩・砂の4層であり、地表材質の重みのRGBA成分の並びと1対1で対応する。
/// アセットコンパイラの地表材質の重み格子もこの定数を読むため、層の数の正本はこの1つである。
pub const 地表層の数: usize = 4;
