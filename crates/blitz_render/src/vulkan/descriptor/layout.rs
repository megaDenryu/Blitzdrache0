//! シーン描画・シャドウ描画・スキニング・布の各パイプラインが共有するディスクリプタセットレイアウトの所有者。
//! binding0-2=combined image sampler(FRAGMENT)、binding3=uniform buffer(VERTEX|FRAGMENT、判断24でビュー射影行列を含むため)、
//! binding4=シャドウマップの比較サンプラー(FRAGMENT、判断35)、binding5=描画対象ユニフォーム(VERTEX|FRAGMENT)。
//! レイアウトの内容は描画対象の個数に依らず同一のため、描画対象の束ごとに作らず1つを共有して束の外が所有する。

use ash::vk;

use crate::error::レンダラーエラー;

pub(crate) struct ディスクリプタレイアウト {
    handle: vk::DescriptorSetLayout,
}

impl ディスクリプタレイアウト {
    pub(crate) fn 生成する(device: &ash::Device) -> Result<Self, レンダラーエラー> {
        let バインド一覧 = [
            テクスチャバインド(0),
            テクスチャバインド(1),
            テクスチャバインド(2),
            vk::DescriptorSetLayoutBinding::default()
                .binding(3)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT),
            テクスチャバインド(4),
            vk::DescriptorSetLayoutBinding::default()
                .binding(5)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT),
        ];
        let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
        // 安全性: deviceは生成済みで有効。
        let handle = unsafe { device.create_descriptor_set_layout(&create_info, None)? };
        Ok(Self { handle })
    }

    /// パイプライン生成とディスクリプタセット割り当てが必要とする生ハンドル。所有権は移らない。
    pub(crate) fn handle(&self) -> vk::DescriptorSetLayout {
        self.handle
    }

    /// 注意: このレイアウトから割り当てたセットを持つディスクリプタプールをすべて破棄した後に呼ぶ。
    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: handleはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe { device.destroy_descriptor_set_layout(self.handle, None) };
    }
}

fn テクスチャバインド(binding: u32) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(binding)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
}
