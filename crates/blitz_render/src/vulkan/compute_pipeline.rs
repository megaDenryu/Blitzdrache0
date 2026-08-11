//! プッシュ定数なしコンピュートパイプラインの汎用生成(判断54の布シミュ9本が使う)。
//! エントリ名を引数に取る点がparticles/skinningの個別実装と異なる。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;

pub(crate) fn 生成する(
    確保係: &GPU資源の確保係<'_>,
    layout: vk::PipelineLayout,
    spirv: &[u8],
    エントリ名: &std::ffi::CStr,
) -> Result<vk::Pipeline, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let モジュール = 確保係.シェーダーモジュールを生成する(spirv)?;
    let stage = vk::PipelineShaderStageCreateInfo::default()
        .stage(vk::ShaderStageFlags::COMPUTE)
        .module(モジュール)
        .name(エントリ名);
    let create_info = vk::ComputePipelineCreateInfo::default().stage(stage).layout(layout);
    // 安全性: stage・layoutは構築済み・生成済みの値のみを参照し、deviceは生成済みで有効。
    let 生成結果 = unsafe { device.create_compute_pipelines(vk::PipelineCache::null(), &[create_info], None) };
    // 安全性: モジュールはパイプライン生成呼び出しの間だけ必要で、生成後は破棄してよい。
    unsafe { device.destroy_shader_module(モジュール, None) };
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
