//! ベイク済み画像の形と、それがVulkanの画像種別・ビュー種別・範囲へどう写るかを所有するモジュール。
//!
//! 深さ1の平面を立体の特別な場合として表さないのは、画像種別とビュー種別が形で変わり、深さ1の立体画像と
//! 平面画像がVulkanでは別物だからである。マジック値(深さ1なら平面)で分けると、確保・ビュー・コピーの3箇所が
//! 同じ暗黙の規約を別々に読むことになる。

use ash::vk;

/// ベイク済み画像の形。透過率・多重散乱・スカイビューは平面、空中遠近は立体である。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 大気のベイク済み画像の形 {
    平面 { 幅: u32, 高さ: u32 },
    立体 { 幅: u32, 高さ: u32, 奥行き: u32 },
}

impl 大気のベイク済み画像の形 {
    pub(crate) fn 画像種別(self) -> vk::ImageType {
        match self {
            Self::平面 { .. } => vk::ImageType::TYPE_2D,
            Self::立体 { .. } => vk::ImageType::TYPE_3D,
        }
    }

    pub(crate) fn ビュー種別(self) -> vk::ImageViewType {
        match self {
            Self::平面 { .. } => vk::ImageViewType::TYPE_2D,
            Self::立体 { .. } => vk::ImageViewType::TYPE_3D,
        }
    }

    pub(crate) fn 範囲(self) -> vk::Extent3D {
        match self {
            Self::平面 { 幅, 高さ } => vk::Extent3D {
                width: 幅,
                height: 高さ,
                depth: 1,
            },
            Self::立体 { 幅, 高さ, 奥行き } => vk::Extent3D {
                width: 幅,
                height: 高さ,
                depth: 奥行き,
            },
        }
    }

    /// レンダーグラフへ登録するときに渡す寸法。グラフはこの値を描画領域にしか使わないため、
    /// コンピュートとコピーだけが触れるベイク済み画像では読まれない。立体の画像は横と縦を渡す。
    pub(crate) fn グラフへ渡す寸法(self) -> vk::Extent2D {
        let 範囲 = self.範囲();
        vk::Extent2D {
            width: 範囲.width,
            height: 範囲.height,
        }
    }
}
