//! 梱包工程へ渡す、材質1件の1つの役割に割り当てるテクスチャの指定。担当するのは、テクスチャIDと画像IDと画素を
//! 1つの値として運び、台帳が重複除去に使う画像の同一性を欠かさないことである。
//!
//! 画素を借用で持つのは、材質のCPU側データがGPUへ載せ終わるまでの寿命であり、世代の構築がその複製を持たないためである。

use crate::texture_material::テクスチャ素材;

use super::image_id::画像ID;
use super::texture_id::テクスチャID;

pub(crate) struct テクスチャ指定<'素材> {
    テクスチャid: テクスチャID,
    画像id: 画像ID,
    素材: &'素材 テクスチャ素材,
}

impl<'素材> テクスチャ指定<'素材> {
    pub(crate) fn 生成する(テクスチャid: テクスチャID, 画像id: 画像ID, 素材: &'素材 テクスチャ素材) -> Self {
        Self {
            テクスチャid, 画像id, 素材
        }
    }

    pub(crate) fn テクスチャid(&self) -> テクスチャID {
        self.テクスチャid
    }

    pub(crate) fn 画像id(&self) -> 画像ID {
        self.画像id
    }

    pub(crate) fn 素材(&self) -> &'素材 テクスチャ素材 {
        self.素材
    }
}
