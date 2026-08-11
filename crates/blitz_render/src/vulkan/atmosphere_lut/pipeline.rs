//! 大気のベイク済み画像を焼くコンピュートパイプラインと、そのパイプラインレイアウトの組。
//! プッシュ定数を持たず、束縛するのはディスクリプタセット1つだけである(解像度はシェーダーが書き込み先へ問い合わせる)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::compute_pipeline;

/// そのパイプラインが受け取る即時定数の枠。持たないパスと持つパスを型で分ける。
/// バイト数0で「無し」を表さないのは、0の範囲を宣言したレイアウトと範囲を持たないレイアウトが別物であり、
/// 前者はvalidationが値域外として指摘するためである。
#[derive(Debug, Clone, Copy)]
pub(in crate::vulkan) enum 即時定数の枠 {
    無し,
    バイト数(u32),
}

pub(in crate::vulkan) struct 生成パイプライン {
    pub(in crate::vulkan) handle: vk::Pipeline,
    pub(in crate::vulkan) layout: vk::PipelineLayout,
}

impl 生成パイプライン {
    pub(in crate::vulkan) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        ディスクリプタlayout: vk::DescriptorSetLayout,
        押し込み: 即時定数の枠,
        spirv: &[u8],
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let layout = レイアウトを作る(device, ディスクリプタlayout, 押し込み)?;
        match compute_pipeline::生成する(確保係, layout, spirv, c"computeMain") {
            Ok(handle) => Ok(Self { handle, layout }),
            Err(誤り) => {
                // 安全性: layoutはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_pipeline_layout(layout, None) };
                Err(誤り)
            }
        }
    }

    /// 注意: パイプラインをレイアウトより先に破棄する(パイプラインがレイアウトを参照するため)。
    pub(in crate::vulkan) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: handle・layoutはSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
        unsafe {
            device.destroy_pipeline(self.handle, None);
            device.destroy_pipeline_layout(self.layout, None);
        }
    }
}

fn レイアウトを作る(
    device: &ash::Device,
    ディスクリプタlayout: vk::DescriptorSetLayout,
    押し込み: 即時定数の枠,
) -> Result<vk::PipelineLayout, レンダラーエラー> {
    let set_layouts = [ディスクリプタlayout];
    let 範囲一覧 = match 押し込み {
        即時定数の枠::無し => Vec::new(),
        即時定数の枠::バイト数(バイト数) => vec![
            vk::PushConstantRange::default()
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
                .offset(0)
                .size(バイト数),
        ],
    };
    let create_info = vk::PipelineLayoutCreateInfo::default()
        .set_layouts(&set_layouts)
        .push_constant_ranges(&範囲一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_pipeline_layout(&create_info, None)? })
}
