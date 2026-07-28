//! 可視判定の検収用シーンを焼く入口。担当するのは「どの配置で焼くか」の選択だけであり、
//! 群と床の組み立ては`floor_scene`が、配置の座標は`cull_placement`が持つ。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「段階導入」

use blitz_engine::{アセットID, カタログ, チャンク座標};

use super::{cull_placement, floor_scene};
use crate::compile::コンパイル済みシーン;
use crate::error::アセットコンパイルエラー;

pub fn 植生可視判定シーンをコンパイルする(
    カタログ: &カタログ,
    id: &アセットID,
    所有チャンク: チャンク座標,
) -> Result<コンパイル済みシーン, アセットコンパイルエラー> {
    floor_scene::群と床のシーンをコンパイルする(カタログ, id, 所有チャンク, cull_placement::配置列を作る()?)
}
