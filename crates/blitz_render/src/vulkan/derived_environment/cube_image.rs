//! 派生の立方体画像・メモリ・ビュー一式の所有者。生成は起動時と検査の組み立て時の1回だけであり、
//! 以降のフレームは中身を焼き直すだけで作り直さない。生成の手順は`create`が担い、ここは保持と破棄だけを持つ。
//!
//! 拡散照度と鏡面畳込みが同じ型を使うのは、どちらも「6面ぶんの層を持ち、段ごとに書いて全段を立方体として読む」
//! 同じ形だからである。違うのは段数(拡散は1、鏡面は粗さ段の数)と一辺だけであり、その2つは値で受け取る。
//!
//! ビューを段ごとに持つのは、コンピュートが書き込み先に取れるのが1つの縮小段だけだからである。
//! 消費側の立方体ビューは全段を含み、粗さから段を選んで参照する。

mod create;
mod vulkan_object;

use ash::vk;

use crate::atmosphere::立方体の面数;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::tracked_device::GPUデバイス;

pub(in crate::vulkan) struct 派生の立方体画像 {
    pub(in crate::vulkan) 画像: vk::Image,
    pub(in crate::vulkan) 段ごとの配列ビュー: Vec<vk::ImageView>, // 段番号の順に並べた書き込み先。要素数は段数と等しい
    pub(in crate::vulkan) 立方体ビュー: vk::ImageView,            // 消費側が向きと粗さで参照する先。全段を含む
    最詳細段の一辺: u32,
    段数: u32,
    memory: vk::DeviceMemory,
}

impl 派生の立方体画像 {
    pub(in crate::vulkan) fn 生成する(
        確保係: &GPU資源の確保係<'_>, 最詳細段の一辺: u32, 段数: u32
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(確保係, 最詳細段の一辺, 段数)
    }

    pub(in crate::vulkan) fn 段の一辺(&self, 段: u32) -> u32 {
        self.最詳細段の一辺 >> 段
    }

    /// 1つの段の全6層を走査する計算の発行の大きさ。奥行きが面の層番号である。
    pub(in crate::vulkan) fn 段の範囲(&self, 段: u32) -> vk::Extent3D {
        vk::Extent3D {
            width: self.段の一辺(段),
            height: self.段の一辺(段),
            depth: 1,
        }
    }

    pub(in crate::vulkan) fn 段数(&self) -> u32 {
        self.段数
    }

    pub(in crate::vulkan) fn 層数(&self) -> u32 {
        立方体の面数
    }

    /// グラフへ渡す寸法。グラフはこの値を描画領域にしか使わないため、コンピュートとコピーだけが触れる
    /// この画像では読まれない。
    pub(in crate::vulkan) fn グラフへ渡す寸法(&self) -> vk::Extent2D {
        vk::Extent2D {
            width: self.最詳細段の一辺,
            height: self.最詳細段の一辺,
        }
    }

    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この画像は派生表現一式の1段として呼ばれる(GPU待機済み)。
    pub(in crate::vulkan) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: 画像・全ビュー・memoryはSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
        unsafe {
            device.destroy_image_view(self.立方体ビュー, None);
            for ビュー in &self.段ごとの配列ビュー {
                device.destroy_image_view(*ビュー, None);
            }
            device.destroy_image(self.画像, None);
        }
        device.メモリを解放する(self.memory);
    }
}
