//! 容量を決めるための必要枚数の見積。受け取るのは梱包する材質の一覧と台帳の外で常駐させる枚数で、
//! 返すのはその世代が置くテクスチャの枚数である。
//!
//! 重複除去の鍵を`テクスチャ台帳::引き当てる`と同じ画像の同一性にするのは、見積と実際の発番で違う枚数を出すと、
//! 容量の判定を通った後に発番が容量超過で失敗するためである。

use std::collections::HashSet;

use crate::vulkan::material_table::image_identity::画像同一性;
use crate::vulkan::material_table::pack_input::梱包対象材質;
use crate::vulkan::material_table::texture_role::材質テクスチャ役割;

pub(in crate::vulkan::material_table) fn 必要枚数を数える(材質一覧: &[梱包対象材質<'_>], 台帳外の枚数: u32) -> u32 {
    let mut 同一性集合 = HashSet::new();
    for 材質 in 材質一覧 {
        for 役割 in 材質テクスチャ役割::全役割 {
            if let Some(指定) = 材質.役割の指定(役割) {
                同一性集合.insert(画像同一性::生成する(指定.画像id(), 役割.ビュー契約()));
            }
        }
    }
    let 固有数 = u32::try_from(同一性集合.len()).unwrap_or(u32::MAX);
    固有数.saturating_add(台帳外の枚数)
}
