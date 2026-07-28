//! 地形詳細段選択: 1つの描画束がそのフレームで描く詳細段。束の識別子と段の対であり、束の中身も段の意味もこの型は知らない。
//! 参照: `_doc/設計/地形とカメラ相対描画.md`「LOD」

use crate::draw_bundle_id::描画束ID;
use crate::terrain_detail::地形詳細段;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 地形詳細段選択 {
    束id: 描画束ID,
    段: 地形詳細段,
}

impl 地形詳細段選択 {
    pub fn 生成する(束id: 描画束ID, 段: 地形詳細段) -> Self {
        Self { 束id, 段 }
    }

    pub(crate) fn 束id(self) -> 描画束ID {
        self.束id
    }

    pub(crate) fn 段(self) -> 地形詳細段 {
        self.段
    }
}

/// 束の識別子から段を引く。選択に無い束は最詳細段を描く。段を1つしか持たない束は選択の対象にならず、その束にとって最詳細段が唯一の段だからである。
/// 一覧の件数は常駐チャンク数と同じ数十件であり、束ごとの走査は描画発行の件数と同じ桁に収まる。
pub(crate) fn 段を引く(一覧: &[地形詳細段選択], 束id: 描画束ID) -> 地形詳細段 {
    一覧
        .iter()
        .find(|選択| 選択.束id() == 束id)
        .map_or(地形詳細段::最詳細(), |選択| 選択.段())
}
