//! 世界ごとの目録ソースの置き場の台帳と、対象世界からその置き場を選ぶ引き当て。パスはソースルートからの相対で固定する。
//! 台帳を親から分けるのは、世界が増えるたびにこの一覧だけが伸び、宣言の引き当てと押し合うためである。
//! プロセス境界の綴りの台帳を`argument_name`が持つのと同じ分け方である。

use super::対象世界;

const 板の世界の目録ソース: &str = "chunk_world/chunk_directory.txt";
const 地形の世界の目録ソース: &str = "terrain_world/chunk_directory.txt";
const 植生の世界の目録ソース: &str = "vegetation_world/chunk_directory.txt";
const 見本の集落の世界の目録ソース: &str = "village_world/chunk_directory.txt";
const 目視見本の世界の目録ソース: &str = "terrain_visual_world/chunk_directory.txt";
const ブロック圧縮の対照世界の目録ソース: &str = "texture_compression_world/chunk_directory.txt";
const 場所巡りの世界の目録ソース: &str = "fox_tour_world/chunk_directory.txt";

/// 頂点診断の世界が地形の目録を読むのは、代表世界と同じ25チャンクの同じ地面を対象にするためである。
pub(super) fn 選ぶ(世界: 対象世界) -> &'static str {
    match 世界 {
        対象世界::板の世界 => 板の世界の目録ソース,
        対象世界::地形の世界 | 対象世界::頂点診断の世界(_) => 地形の世界の目録ソース,
        対象世界::植生の世界 => 植生の世界の目録ソース,
        対象世界::見本の集落の世界 => 見本の集落の世界の目録ソース,
        対象世界::目視見本の世界 => 目視見本の世界の目録ソース,
        対象世界::ブロック圧縮の対照世界 => ブロック圧縮の対照世界の目録ソース,
        対象世界::場所巡りの世界 => 場所巡りの世界の目録ソース,
    }
}
