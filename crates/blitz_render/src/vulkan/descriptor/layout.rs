//! シーン描画・シャドウ描画・スキニング・布の各パイプラインが共有するディスクリプタセットレイアウトの所有者。
//! binding0-2=combined image sampler(FRAGMENT)、binding3=ビュー・シーンパス定数(VERTEX|FRAGMENT、ビュー射影行列を含むため)、
//! binding4=シャドウマップの比較サンプラー(FRAGMENT、判断35)、binding5=材質レコードのストレージバッファ(FRAGMENT。係数を読むのは画素段だけである)、
//! binding6=個体変換のストレージバッファ(VERTEX)、binding7=可視ID列のストレージバッファ(VERTEX)、
//! binding8=多段影定数(VERTEX|FRAGMENT。シャドウ記録の頂点段とシーンの画素段が同じ1本を読む)、
//! binding9=空パス定数(VERTEX|FRAGMENT。空パスと空中遠近の合成パスだけが読む)。
//! 番号の意味は`shaders/view_pass_uniform.slang`・`cascade_shadow_uniform.slang`・`sky_pass_uniform.slang`の宣言が正本である。
//! シーンの頂点シェーダーは`SV_InstanceID`で可視ID列を参照し、得た添字で個体変換を参照する。シャドウの頂点シェーダーは
//! 可視ID列を使わず個体変換を直に参照する(参照: `_doc/設計/植生インスタンスと物量計測.md`「可視判定」)。
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
            シェーダー定数バインド(3),
            テクスチャバインド(4),
            ストレージバッファバインド(5, vk::ShaderStageFlags::FRAGMENT),
            ストレージバッファバインド(6, vk::ShaderStageFlags::VERTEX),
            ストレージバッファバインド(7, vk::ShaderStageFlags::VERTEX),
            シェーダー定数バインド(8),
            シェーダー定数バインド(9),
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

fn シェーダー定数バインド(binding: u32) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(binding)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT)
}

/// 読むステージを引数で受けるのは、個体変換と可視ID列を頂点段が、材質レコードを画素段が読むためである。
fn ストレージバッファバインド(binding: u32, ステージ: vk::ShaderStageFlags) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(binding)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .descriptor_count(1)
        .stage_flags(ステージ)
}

fn テクスチャバインド(binding: u32) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(binding)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
}
