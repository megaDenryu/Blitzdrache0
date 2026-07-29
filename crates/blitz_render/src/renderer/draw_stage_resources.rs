//! 描画段階がGPUへ発行するときに束縛するパイプラインと、そのパイプラインレイアウトの所有者。
//! シーン段階・影段階が使う現物を具体フィールドで持ち、段階の種類を鍵にした表では持たない。
//! 段階が増えるたびにレンダラー直下のフィールドが増えることを止めるためにこの型がある
//! (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「描画段階資源の器と布専用シャドウ経路」)。
//! 破棄はレンダラーの単一破棄元(`renderer/destroy.rs`)がこの型の`破棄する`を1回呼ぶ形を保つ。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_bundle::シェーダー束;
use crate::vulkan;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 描画段階資源 {
    シーン: vulkan::pipeline::パイプライン,
    シャドウ: vulkan::pipeline::シャドウパイプライン,
}

impl 描画段階資源 {
    /// `シーンカラー形式`はシーン段階の色アタッチメントの形式(ポスト処理があればHDR中間画像、無ければスワップチェーン)。
    /// 影段階は深度だけへ書くため色形式を要らない。
    pub(super) fn 生成する(
        device: &GPUデバイス,
        シーンカラー形式: vk::Format,
        ディスクリプタlayout: vk::DescriptorSetLayout,
        シェーダー: &シェーダー束,
    ) -> Result<Self, レンダラーエラー> {
        let シーン = vulkan::pipeline::パイプライン::生成する(
            device,
            シーンカラー形式,
            vulkan::depth::深度形式,
            ディスクリプタlayout,
            &シェーダー.シーン,
        )?;
        match vulkan::pipeline::シャドウパイプライン::生成する(device, ディスクリプタlayout, &シェーダー.シャドウ) {
            Ok(シャドウ) => Ok(Self { シーン, シャドウ }),
            Err(誤り) => {
                シーン.破棄する(device);
                Err(誤り)
            }
        }
    }

    pub(super) fn シーンpipeline(&self) -> vk::Pipeline {
        self.シーン.handle
    }

    pub(super) fn シーンlayout(&self) -> vk::PipelineLayout {
        self.シーン.layout
    }

    pub(super) fn シャドウpipeline(&self) -> vk::Pipeline {
        self.シャドウ.handle
    }

    pub(super) fn シャドウlayout(&self) -> vk::PipelineLayout {
        self.シャドウ.layout
    }

    /// ホットリロードでシーンのパイプラインだけを入れ替える。
    /// 前提: 呼び出し元が旧パイプラインのGPU使用完了を待ってから呼ぶ(`renderer/replace_shader.rs`)。
    pub(super) fn シーンを差し替える(&mut self, device: &GPUデバイス, 新パイプライン: vulkan::pipeline::パイプライン) {
        self.シーン.破棄する(device);
        self.シーン = 新パイプライン;
    }

    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この束はその1段として呼ばれる(GPU待機済み)。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.シーン.破棄する(device);
        self.シャドウ.破棄する(device);
    }
}
