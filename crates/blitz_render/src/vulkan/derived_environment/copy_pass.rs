//! 派生表現まわりのコピーのパス宣言を束ねるモジュール。画像から画像への複製は`mip_zero_copy`、
//! 画像からホスト可視バッファへの写しは`readback_pass`、ホスト可視バッファから画像への書き込みは`upload_pass`が持つ。
//!
//! 部分範囲の組み立てだけをここに置くのは、複製と読み戻しがどちらも同じ「段と層の指定」を要り、
//! 2箇所で書くと段や層数の取り違えが片方だけで起こるためである。

mod mip_zero_copy;
mod readback_pass;
mod upload_pass;

use ash::vk;

pub(in crate::vulkan) use mip_zero_copy::最詳細段の複製を作る;
pub(in crate::vulkan) use readback_pass::{立方体の読み戻しを作る, 表の読み戻しを作る};
pub(in crate::vulkan) use upload_pass::書き込みパスを作る;

/// 立方体の派生画像の1テクセルのバイト数。4成分の半精度である。
pub(super) const 四成分テクセルのバイト数: u64 = 8;

pub(super) fn 層の部分範囲(段: u32, 層数: u32) -> vk::ImageSubresourceLayers {
    vk::ImageSubresourceLayers::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .mip_level(段)
        .base_array_layer(0)
        .layer_count(層数)
}
