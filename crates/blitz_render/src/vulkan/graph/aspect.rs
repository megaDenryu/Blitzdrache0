//! 画像アスペクト(カラー/深度)。バリア発行時のImageSubresourceRange構築に使う。

use ash::vk;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 画像アスペクト {
    カラー,
    深度,
}

impl 画像アスペクト {
    /// このアスペクトの画像全体(縮小段0、全レイヤー)を指す部分範囲。
    /// レイヤーを全部含めるのは、シャドウマップが距離区分ごとの層を持つ配列画像であり、
    /// グラフが配列全体を1資源として追跡するためである。
    pub(crate) fn 部分範囲(&self) -> vk::ImageSubresourceRange {
        let aspect_mask = match self {
            Self::カラー => vk::ImageAspectFlags::COLOR,
            Self::深度 => vk::ImageAspectFlags::DEPTH,
        };
        vk::ImageSubresourceRange::default()
            .aspect_mask(aspect_mask)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(vk::REMAINING_ARRAY_LAYERS)
    }
}
