//! パイプライン生成結果からハンドルを取り出し、失敗時はlayoutを片付ける。

use ash::vk;

use crate::error::レンダラーエラー;

pub(crate) fn 取り出す(
    device: &ash::Device,
    layout: vk::PipelineLayout,
    生成結果: Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::Result)>,
) -> Result<(vk::Pipeline, vk::PipelineLayout), レンダラーエラー> {
    match 生成結果 {
        Ok(一覧) => {
            let Some(&handle) = 一覧.first() else {
                panic!("create_graphics_pipelinesが成功したのにパイプラインが0本だった(Vulkan実装の契約違反)");
            };
            Ok((handle, layout))
        }
        Err((_, 誤り)) => {
            // 安全性: パイプライン生成に失敗したため、layoutを参照するパイプラインは存在しない。
            unsafe { device.destroy_pipeline_layout(layout, None) };
            Err(誤り.into())
        }
    }
}
