//! テクスチャ台帳が重複除去の鍵に使う画像の同一性。担当するのは、「同じ画像を同じビュー契約で読むなら1枚で足りる」という
//! 判定の材料を1つの値にすることである。
//!
//! ビュー契約を鍵へ含めるのは、同じ画素でも色として読むか線形データとして読むかで画像形式が変わり、
//! 同じ画像ビューを共有できないためである。

use crate::texture_material::テクスチャ用途;

use super::image_id::画像ID;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::vulkan::material_table) struct 画像同一性 {
    画像id: 画像ID,
    ビュー契約: テクスチャ用途,
}

impl 画像同一性 {
    pub(in crate::vulkan::material_table) const fn 生成する(画像id: 画像ID, ビュー契約: テクスチャ用途) -> Self {
        Self { 画像id, ビュー契約 }
    }
}
