//! 生成側のセットが持つ束縛の並びと、そこへ現物を書き込む先。触れるのは番号と種別の対応だけであり、
//! プールもパイプラインも知らない。
//!
//! 番号を並びの位置そのものにするのは、`shaders/cluster_light_assignment.slang`の宣言が0から連番だからである。
//! 並びはヘッダ・局所光レコード列・クラスタ格子・クラスタ光添字列であり、前の2つは照明問い合わせのセットが
//! 画素段へ結ぶのと同じバッファを生成側へも結んだものである。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{ディスクリプタの書き込み先, 束縛番号, 照明問い合わせのバッファ組};

/// 束縛の種別の並び。位置がそのまま番号になる。
pub(super) const 束縛の種別一覧: [vk::DescriptorType; 4] = [
    vk::DescriptorType::UNIFORM_BUFFER,
    vk::DescriptorType::STORAGE_BUFFER,
    vk::DescriptorType::STORAGE_BUFFER,
    vk::DescriptorType::STORAGE_BUFFER,
];

/// 1つの選別のセットへ現物を書き込む先。
pub(super) struct クラスタ選別のセットの書き込み先<'書き込み>(ディスクリプタの書き込み先<'書き込み>);

impl<'書き込み> クラスタ選別のセットの書き込み先<'書き込み> {
    pub(super) fn 生成する(device: &'書き込み ash::Device, セット: vk::DescriptorSet) -> Self {
        Self(ディスクリプタの書き込み先::生成する(device, セット))
    }

    /// 前提: 呼び出し時点でGPUがこのセットを使用していないこと(生成直後)。
    pub(super) fn バッファ組を結ぶ(&self, バッファ組: 照明問い合わせのバッファ組) {
        let 並び = [
            バッファ組.ヘッダ,
            バッファ組.局所光列,
            バッファ組.クラスタ格子,
            バッファ組.クラスタ光添字列,
        ];
        for (位置, buffer) in 並び.into_iter().enumerate() {
            self.0.バッファ全体を結ぶ(束縛番号にする(位置), 束縛の種別一覧[位置], buffer);
        }
    }
}

pub(super) fn セットレイアウトを作る(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧: Vec<vk::DescriptorSetLayoutBinding<'_>> = 束縛の種別一覧
        .iter()
        .enumerate()
        .map(|(位置, &種別)| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(束縛番号にする(位置).gpu境界値())
                .descriptor_type(種別)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
        })
        .collect();
    let 生成情報 = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_set_layout(&生成情報, None)? })
}

fn 束縛番号にする(位置: usize) -> 束縛番号 {
    束縛番号::生成する(u32::try_from(位置).unwrap_or_else(|_| panic!("選別の束縛番号がu32に収まらない: {位置}")))
}
