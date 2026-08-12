//! 明るさの圧縮用ディスクリプタ: binding0=HDR画像・binding1=光のにじみ結果のcombined image sampler 2個と、
//! binding2=GPU上の露出状態のストレージバッファ1個のlayout・pool・set。
//!
//! 露出状態を露出方式に関わらず束縛するのは、世界ごとにセットの形が変わると束縛の一致条件が1つ増えるためである。
//! 時刻別固定の枝は即時定数の分岐でこのバッファを1度も読まない。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{
    宣言から作ったセットレイアウト, 宣言から割り当てたセット, 宣言した束縛の並び, 束縛番号
};

const 画素段: vk::ShaderStageFlags = vk::ShaderStageFlags::FRAGMENT;

/// 束縛の並び。HDR画像・光のにじみ結果・GPU上の露出状態の順である。
pub(super) const 束縛の宣言: 宣言した束縛の並び<3> = 宣言した束縛の並び::生成する([
    (束縛番号::生成する(0), vk::DescriptorType::COMBINED_IMAGE_SAMPLER, 画素段),
    (束縛番号::生成する(1), vk::DescriptorType::COMBINED_IMAGE_SAMPLER, 画素段),
    (束縛番号::生成する(2), vk::DescriptorType::STORAGE_BUFFER, 画素段),
]);

pub(super) struct 明るさの圧縮ディスクリプタ {
    pub(super) layout: 宣言から作ったセットレイアウト<3>,
    pub(super) pool: vk::DescriptorPool,
    pub(super) set: 宣言から割り当てたセット<3>,
}

impl 明るさの圧縮ディスクリプタ {
    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: poolはこの構造体が唯一の所有者であり、その破棄がsetの解放を暗黙に行う。
        unsafe { device.destroy_descriptor_pool(self.pool, None) };
        self.layout.破棄する(device);
    }
}

pub(super) fn 生成する(device: &ash::Device) -> Result<明るさの圧縮ディスクリプタ, レンダラーエラー> {
    let layout = 束縛の宣言.セットレイアウトを確保する(device)?;

    let pool_size一覧 = [
        vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(2),
        vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1),
    ];
    let pool_info = vk::DescriptorPoolCreateInfo::default().max_sets(1).pool_sizes(&pool_size一覧);
    // 安全性: deviceは生成済みで有効。失敗時はlayoutを片付ける。
    let pool = match unsafe { device.create_descriptor_pool(&pool_info, None) } {
        Ok(pool) => pool,
        Err(誤り) => {
            layout.破棄する(device);
            return Err(誤り.into());
        }
    };

    match layout.プールからセットを割り当てる(device, pool, 1) {
        Ok(一覧) => {
            let Some(set) = 一覧.into_iter().next() else {
                panic!("要求した1つのセットが返らなかった");
            };
            Ok(明るさの圧縮ディスクリプタ { layout, pool, set })
        }
        Err(誤り) => {
            // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_pool(pool, None) };
            layout.破棄する(device);
            Err(誤り)
        }
    }
}
