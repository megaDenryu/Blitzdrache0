//! プッシュ定数なしコンピュートパイプラインの生成。布シミュレーション・大気のベイク済み画像・自動露出・
//! クラスタ割り当て・局所可視性が使い、エントリ名だけがパスごとに異なる。
//!
//! シェーダーモジュールの生成と同じ階層へ置くのは、生成に要る材料が論理デバイスとSPIR-Vのバイト列だけであり、
//! ここで作ったモジュールを呼び出し元へ渡さずその場で破棄できるためである。

use ash::vk;

use super::GPU資源の確保係;
use crate::error::レンダラーエラー;

impl GPU資源の確保係<'_> {
    pub(crate) fn コンピュートパイプラインを生成する(
        &self,
        layout: vk::PipelineLayout,
        spirv: &[u8],
        エントリ名: &std::ffi::CStr,
    ) -> Result<vk::Pipeline, レンダラーエラー> {
        let モジュール = self.シェーダーモジュールを生成する(spirv)?;
        let stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::COMPUTE)
            .module(モジュール)
            .name(エントリ名);
        let create_info = vk::ComputePipelineCreateInfo::default().stage(stage).layout(layout);
        // 安全性: stage・layoutは構築済み・生成済みの値のみを参照し、deviceは生成済みで有効。
        let 生成結果 = unsafe { self.device.create_compute_pipelines(vk::PipelineCache::null(), &[create_info], None) };
        // 安全性: モジュールはパイプライン生成呼び出しの間だけ必要で、生成後は破棄してよい。
        unsafe { self.device.destroy_shader_module(モジュール, None) };
        match 生成結果 {
            Ok(一覧) => {
                let Some(&handle) = 一覧.first() else {
                    panic!("create_compute_pipelinesが成功したのにパイプラインが0本だった(Vulkan実装の契約違反)");
                };
                Ok(handle)
            }
            Err((_, 誤り)) => Err(誤り.into()),
        }
    }
}
