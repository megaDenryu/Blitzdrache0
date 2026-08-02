//! フレームごとのシェーダー定数バッファ3本(ビュー定数・多段影定数・空パス定数)の所有者。
//! 3本を1つの型が持つのは、どれも寿命がフレーム×ビューであり、進行中フレームごとの多重化の規律が同じためである。
//! 3本はビューとパスのセット(set0)へまとまる(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。
//!
//! 書き込みタイミングは呼び出し元(renderer/uniform_write.rs)がフェンス待ち後に行う。
//! 空パス定数は空段階を持つフレーム構成でだけ書き、持たない構成では1度も書かない。

mod buffer_set;
pub(crate) mod cascade_bytes;
pub(crate) mod cascade_content;
mod create;
pub(crate) mod sky_bytes;
pub(crate) mod sky_content;
pub(crate) mod sky_write_order;
pub(crate) mod view_bytes;
pub(crate) mod view_content;
pub(crate) mod write_bytes;

#[cfg(test)]
mod layout_tests;
#[cfg(test)]
mod shader_struct_tests;
#[cfg(test)]
mod sky_write_order_tests;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;
use buffer_set::定数バッファ一式;

pub(crate) use cascade_content::多段影定数内容;
pub(crate) use sky_content::空パス定数内容;
pub(crate) use view_content::ビュー定数内容;

pub(crate) struct フレームシェーダー定数一式 {
    ビュー: 定数バッファ一式,
    多段影: 定数バッファ一式,
    空パス: 定数バッファ一式,
}

impl フレームシェーダー定数一式 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, メモリプロパティ)
    }

    pub(crate) fn ビュー定数のbuffer(&self, フレーム添字: フレームスロット添字) -> vk::Buffer {
        self.ビュー.buffer(フレーム添字)
    }

    pub(crate) fn 多段影のbuffer(&self, フレーム添字: フレームスロット添字) -> vk::Buffer {
        self.多段影.buffer(フレーム添字)
    }

    pub(crate) fn 空パスのbuffer(&self, フレーム添字: フレームスロット添字) -> vk::Buffer {
        self.空パス.buffer(フレーム添字)
    }

    pub(crate) fn ビュー定数を書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        内容: &ビュー定数内容,
    ) -> Result<(), レンダラーエラー> {
        self.ビュー.書き込む(device, フレーム添字, &view_bytes::バイト列にする(内容))
    }

    pub(crate) fn 多段影の定数を書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        内容: &多段影定数内容,
    ) -> Result<(), レンダラーエラー> {
        self.多段影.書き込む(device, フレーム添字, &cascade_bytes::バイト列にする(内容))
    }

    pub(crate) fn 空パスの定数を書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        内容: &空パス定数内容,
    ) -> Result<(), レンダラーエラー> {
        self.空パス.書き込む(device, フレーム添字, &sky_bytes::バイト列にする(内容))
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.空パス.破棄する(device);
        self.多段影.破棄する(device);
        self.ビュー.破棄する(device);
    }
}
