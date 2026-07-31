//! 材質スロット割当: 1つの材質スロットIDへ実際の材質を対応させた1件。

use super::material_data::マテリアルデータ;
use super::material_slot_id::材質スロットID;

#[derive(Debug, Clone, PartialEq)]
pub struct 材質スロット割当 {
    pub スロット: 材質スロットID,
    pub マテリアル: マテリアルデータ,
}
