//! 影の視距離の検収用シーンを焼く入口。担当するのは「どの配置で焼くか」の選択だけであり、
//! 群と床の組み立ては`floor_scene`が、配置の座標は`shadow_range_placement`が持つ。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「S2-1: 個体キャスター数の削減(品質方針の裁定)」

use blitz_engine::{アセットID, カタログ, チャンク座標};

use super::{floor_scene, shadow_range_placement};
use crate::compile::コンパイル済みシーン;
use crate::error::アセットコンパイルエラー;

pub fn 植生影視距離シーンをコンパイルする(
    カタログ: &カタログ,
    id: &アセットID,
    所有チャンク: チャンク座標,
) -> Result<コンパイル済みシーン, アセットコンパイルエラー> {
    floor_scene::群と床のシーンをコンパイルする(カタログ, id, 所有チャンク, shadow_range_placement::配置列を作る()?)
}
