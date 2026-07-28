//! 1つの束に属する描画対象ぶんのディスクリプタセットを、その束専用のディスクリプタプールから確保して保持する。
//! 注意: プールの破棄がセットの解放を暗黙に行うため、束の解除はプール1つの破棄で完結する。セット添字もこの型の内側で閉じるため、他の束の追加・解除で添字がずれない。

use ash::vk;

use super::layout::ディスクリプタレイアウト;
use super::{pool, set, shadow_binding};
use crate::error::レンダラーエラー;
use crate::vulkan::object_uniform::描画対象ユニフォーム;
use crate::vulkan::shadow_map::シャドウマップ;
use crate::vulkan::sync::{フレームインフライト数, フレームスロット添字};
use crate::vulkan::texture::マテリアルテクスチャ一式;
use crate::vulkan::uniform::フレームユニフォーム一式;

pub(crate) struct 描画対象ディスクリプタ参照<'a> {
    pub(crate) テクスチャ: &'a マテリアルテクスチャ一式,
    pub(crate) ユニフォーム: &'a 描画対象ユニフォーム,
    /// 個体変換を読むバッファと、そのバイト範囲。個体が1体だけの対象はユニフォームと同じバッファを指す。
    pub(crate) 個体変換: (vk::Buffer, vk::DeviceSize),
}

pub(crate) struct 描画対象ディスクリプタプール {
    pool: vk::DescriptorPool,
    set一覧: Vec<vk::DescriptorSet>,
}

impl 描画対象ディスクリプタプール {
    pub(crate) fn 生成する(
        device: &ash::Device,
        レイアウト: &ディスクリプタレイアウト,
        描画対象一覧: &[描画対象ディスクリプタ参照<'_>],
        ユニフォーム: &フレームユニフォーム一式,
        シャドウマップ: &シャドウマップ,
    ) -> Result<Self, レンダラーエラー> {
        let pool = pool::生成する(device, 描画対象一覧.len())?;
        let セット数 = 描画対象一覧
            .len()
            .checked_mul(フレームインフライト数)
            .unwrap_or_else(|| panic!("ディスクリプタセット数がusizeを超えた"));
        let set一覧 = match set::割り当てる(device, pool, レイアウト.handle(), セット数) {
            Ok(set一覧) => set一覧,
            Err(誤り) => {
                // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_pool(pool, None) };
                return Err(誤り);
            }
        };
        全セットの内容を書き込む(device, &set一覧, 描画対象一覧, ユニフォーム, シャドウマップ);
        Ok(Self { pool, set一覧 })
    }

    /// 注意: 2つの添字は掛け合わせた1本の配列を引くため、入れ替えると別の描画対象のセットを引いたまま描画が成立する。
    /// フレームスロット添字を別型にすることで、描画対象添字との入れ替えをコンパイルエラーにする。
    pub(crate) fn set(&self, 描画対象添字: usize, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        let 添字 = 描画対象添字 * フレームインフライト数 + フレーム添字.配列添字();
        match self.set一覧.get(添字) {
            Some(set) => *set,
            None => panic!("描画対象またはフレーム添字がディスクリプタセット一覧の範囲外だった"),
        }
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: poolの破棄がsetの解放を暗黙に行う。poolはSelfが唯一の所有者であり、
        // 破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe { device.destroy_descriptor_pool(self.pool, None) };
    }
}

fn 全セットの内容を書き込む(
    device: &ash::Device,
    set一覧: &[vk::DescriptorSet],
    描画対象一覧: &[描画対象ディスクリプタ参照<'_>],
    ユニフォーム: &フレームユニフォーム一式,
    シャドウマップ: &シャドウマップ,
) {
    for (描画対象添字, 描画対象) in 描画対象一覧.iter().enumerate() {
        for フレーム添字 in フレームスロット添字::全スロット() {
            let set = set一覧[描画対象添字 * フレームインフライト数 + フレーム添字.配列添字()];
            set::テクスチャバインディングを書き込む(device, set, 描画対象.テクスチャ);
            set::ユニフォームバインディングを書き込む(device, set, ユニフォーム.buffer(フレーム添字));
            set::描画対象ユニフォームを書き込む(device, set, 描画対象.ユニフォーム.buffer);
            set::個体変換を書き込む(device, set, 描画対象.個体変換.0, 描画対象.個体変換.1);
            shadow_binding::シャドウマップバインディングを書き込む(device, set, シャドウマップ);
        }
    }
}
