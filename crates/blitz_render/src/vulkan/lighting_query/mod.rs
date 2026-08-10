//! 照明問い合わせのセット(set3)の資源の所有者。進行中フレームスロットごとに、ヘッダの定数バッファ・
//! 方向光レコード列・局所光レコード列・ディスクリプタセットを1組ずつ持つ。
//!
//! スロットごとに分けるのは、単一のセットを毎フレーム上書きするとGPUが読んでいる最中のバッファを書き換えるためである。
//! 書けるのはそのスロットのフェンスを通過した後だけであり、描画はそのスロットのセットをパスごとに1回束縛する
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段5)。
//! 多段シャドウマップは束縛先が変わらない固定資源のため、全スロットのセットが同じ画像を生成時に1度だけ結ぶ。

pub(crate) mod capacity;
mod cluster_bind;
mod cluster_buffers;
mod cluster_readback;
mod create;
pub(crate) mod directional_bytes;
pub(crate) mod directional_content;
mod distant_environment_bind;
pub(crate) mod header_bytes;
pub(crate) mod header_content;
pub(crate) mod local_bytes;
pub(crate) mod local_content;
pub(crate) mod local_sequence;
mod local_visibility_bind;
pub(crate) mod pack;
mod slot_resources;
mod writable_buffer;

#[cfg(test)]
mod directional_layout_tests;
#[cfg(test)]
mod local_layout_tests;
#[cfg(test)]
mod pack_tests;
#[cfg(test)]
mod record_byte_read;
#[cfg(test)]
mod shader_struct_tests;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{lighting_set, シーンセットレイアウト一式};
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::{pipeline_ledger::照明束縛レイアウト, shadow_resources::影の資源の組, tracked_device::GPUデバイス};
use {pack::照明問い合わせのバイト列, slot_resources::スロット資源};

pub(crate) use header_content::{ヘッダの間接照明, 照明問い合わせヘッダ内容};
pub(crate) use {
    directional_content::方向光レコード内容, local_content::局所光レコード内容, local_sequence::局所光レコードの並び
};

pub(crate) struct 照明問い合わせ資源束 {
    pool: vk::DescriptorPool,
    スロット一覧: Vec<スロット資源>,
    /// このセットが宣言した枝。遠方環境の3つの画像を結ぶかどうかをこの値だけが決める。
    束縛レイアウト: 照明束縛レイアウト,
    /// 遠方環境の枝のときだけ持つ、3つの画像を読む固定サンプラー。
    遠方環境サンプラー: Option<vk::Sampler>,
}

impl 照明問い合わせ資源束 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        レイアウト: &シーンセットレイアウト一式,
        影の資源: &影の資源の組,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, メモリプロパティ, レイアウト, 影の資源)
    }

    /// 注意: 呼び出し元はこのスロットの描画完了フェンスを待ってから呼ぶ(renderer/uniform_write.rsの経路)。
    pub(crate) fn スロットへ書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        ヘッダ: &照明問い合わせヘッダ内容,
        方向光一覧: &[方向光レコード内容],
        局所光一覧: &局所光レコードの並び,
    ) -> Result<(), レンダラーエラー> {
        let バイト列: 照明問い合わせのバイト列 = pack::照明問い合わせのバイト列を組み立てる(ヘッダ, 方向光一覧, 局所光一覧)?;
        self.スロット一覧[フレーム添字.配列添字()].書き込む(device, &バイト列)
    }

    pub(crate) fn セット(&self, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        self.スロット一覧[フレーム添字.配列添字()].セット
    }

    /// 注意: プールの破棄がセットの解放を暗黙に行う。バッファは確保の逆順に破棄する。
    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この束はその1段として呼ばれる(GPU待機済み)。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        if let Some(サンプラー) = self.遠方環境サンプラー {
            lighting_set::distant_environment::サンプラーを破棄する(device, サンプラー);
        }
        for スロット in self.スロット一覧.iter().rev() {
            スロット.破棄する(device);
        }
        // 安全性: poolはSelfが唯一の所有者であり、破棄時点でGPU側の使用完了を呼び出し元が保証する。
        unsafe { device.destroy_descriptor_pool(self.pool, None) };
    }
}
