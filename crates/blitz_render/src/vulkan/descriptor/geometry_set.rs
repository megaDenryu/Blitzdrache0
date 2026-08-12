//! set1(ジオメトリと可視のセット)のレイアウトと、そのセットへ2本のストレージバッファを結ぶ書き込み先。触れるのは
//! 個体レコード(binding0)と可視ID列(binding1)の2つだけである。
//! 2本を同じセットへ置くのは、どちらも頂点段が描画対象ごとに読み、束縛の単位が描画対象で一致するためである
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。
//! 番号の正本は`shaders/instance_transform.slang`・`visible_id.slang`の宣言である。

use ash::vk;

use super::{ディスクリプタの書き込み先, 束縛番号};
use crate::error::レンダラーエラー;
use crate::vulkan::instance_transform::個体レコード参照;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::visible_id::可視ID列参照;

pub(crate) const 個体レコードの束縛番号: 束縛番号 = 束縛番号::生成する(0);
pub(crate) const 可視ID列の束縛番号: 束縛番号 = 束縛番号::生成する(1);

/// 1つのジオメトリと可視のセットへ現物を書き込む先。
pub(super) struct ジオメトリと可視のセットの書き込み先<'書き込み>(ディスクリプタの書き込み先<'書き込み>);

impl<'書き込み> ジオメトリと可視のセットの書き込み先<'書き込み> {
    pub(super) fn 生成する(device: &'書き込み ash::Device, セット: vk::DescriptorSet) -> Self {
        Self(ディスクリプタの書き込み先::生成する(device, セット))
    }

    /// 個体が1体だけの対象も1要素ぶんの範囲を持つ専用のバッファを結び、可視ID列はそのフレームスロットのバッファを結ぶ。
    pub(super) fn 個体レコードと可視id列を結ぶ(
        &self,
        個体レコード: &個体レコード参照,
        可視id列: &可視ID列参照,
        フレーム添字: フレームスロット添字,
    ) {
        let 対応 = [
            (個体レコードの束縛番号, 個体レコード.buffer(フレーム添字), 個体レコード.範囲()),
            (可視ID列の束縛番号, 可視id列.buffer(フレーム添字), 可視id列.範囲()),
        ];
        for (番号, buffer, 範囲) in 対応 {
            self.0
                .バッファの先頭からの範囲を結ぶ(番号, vk::DescriptorType::STORAGE_BUFFER, buffer, 範囲);
        }
    }
}

pub(super) fn レイアウトを生成する(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧 = [
        頂点段のストレージバッファバインド(個体レコードの束縛番号),
        頂点段のストレージバッファバインド(可視ID列の束縛番号),
    ];
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}

fn 頂点段のストレージバッファバインド(番号: 束縛番号) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(番号.gpu境界値())
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::VERTEX)
}
