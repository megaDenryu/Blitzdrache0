//! プロセス境界で世界を指す綴りの台帳と、綴りから対象世界を選ぶ解析。
//! 綴りはxtask側(`xtask/src/compile_assets/world_name.rs`)にも同じものがあり、2つの実行ファイルの
//! 引数の受け渡しがこの綴りで一致していることを、知らない綴りを失敗にすることで確かめる。

use super::part_house_row_declaration::家の並びの規模;
use super::vertex_diagnostic_declaration::{粗い原型, 細かい原型};
use super::対象世界;

const 板の世界: &str = "chunk_world";
const 地形の世界: &str = "terrain_world";
const 植生の世界: &str = "vegetation_world";
const 見本の集落の世界: &str = "village_world";
const 部品で建てた十軒の世界: &str = "part_house_row_ten_world";
const 部品で建てた百軒の世界: &str = "part_house_row_hundred_world";
const 部品で組んだ木の並びの世界: &str = "part_tree_row_world";
const 目視見本の世界: &str = "terrain_visual_world";
const 頂点診断の粗い世界: &str = "vertex_diag_coarse_world";
const 頂点診断の細かい世界: &str = "vertex_diag_fine_world";
const ブロック圧縮の対照世界: &str = "texture_compression_world";
const 夜の多光源の世界: &str = "night_lights_world";
const 屋内の多光源の世界: &str = "stone_hut_world";
const 場所巡りの世界: &str = "fox_tour_world";

const 有効な綴り: [&str; 14] = [
    板の世界,
    地形の世界,
    植生の世界,
    見本の集落の世界,
    部品で建てた十軒の世界,
    部品で建てた百軒の世界,
    部品で組んだ木の並びの世界,
    目視見本の世界,
    頂点診断の粗い世界,
    頂点診断の細かい世界,
    ブロック圧縮の対照世界,
    夜の多光源の世界,
    屋内の多光源の世界,
    場所巡りの世界,
];

pub(super) fn 解析する(引数名: &str) -> Result<対象世界, String> {
    match 引数名 {
        板の世界 => Ok(対象世界::板の世界),
        地形の世界 => Ok(対象世界::地形の世界),
        植生の世界 => Ok(対象世界::植生の世界),
        見本の集落の世界 => Ok(対象世界::見本の集落の世界),
        部品で建てた十軒の世界 => Ok(対象世界::部品で組んだ家の並びの世界(家の並びの規模::十軒)),
        部品で建てた百軒の世界 => Ok(対象世界::部品で組んだ家の並びの世界(家の並びの規模::百軒)),
        部品で組んだ木の並びの世界 => Ok(対象世界::部品で組んだ木の並びの世界),
        目視見本の世界 => Ok(対象世界::目視見本の世界),
        頂点診断の粗い世界 => Ok(対象世界::頂点診断の世界(粗い原型)),
        頂点診断の細かい世界 => Ok(対象世界::頂点診断の世界(細かい原型)),
        ブロック圧縮の対照世界 => Ok(対象世界::ブロック圧縮の対照世界),
        夜の多光源の世界 => Ok(対象世界::夜の多光源の世界),
        屋内の多光源の世界 => Ok(対象世界::屋内の多光源の世界),
        場所巡りの世界 => Ok(対象世界::場所巡りの世界),
        他 => Err(format!("未知の世界名である: {他}(有効な値は{})", 有効な綴り.join("と"))),
    }
}
