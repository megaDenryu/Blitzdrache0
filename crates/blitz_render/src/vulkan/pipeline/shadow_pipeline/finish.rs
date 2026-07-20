//! `create_graphics_pipelines`の生成結果からシャドウパイプラインを取り出す。
//! 失敗時はlayoutの後始末（破棄）まで行う(グラフィックスパイプラインと同じ様式)。

use ash::vk;

use super::super::シャドウパイプライン;
use crate::error::レンダラーエラー;

pub(super) fn パイプラインを取り出す(
    device: &ash::Device,
    layout: vk::PipelineLayout,
    生成結果: Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::Result)>,
) -> Result<シャドウパイプライン, レンダラーエラー> {
    match 生成結果 {
        Ok(一覧) => {
            let Some(&handle) = 一覧.first() else {
                // create_graphics_pipelinesが成功を返したのにパイプラインが0本なのは
                // Vulkan実装がその契約を破っている状態であり回復不能。
                panic!("create_graphics_pipelinesが成功したのにシャドウパイプラインが0本だった");
            };
            Ok(シャドウパイプライン { handle, layout })
        }
        Err((_, 誤り)) => {
            // 安全性: パイプライン生成に失敗したため、layoutを参照するパイプラインは存在しない。
            unsafe { device.destroy_pipeline_layout(layout, None) };
            Err(誤り.into())
        }
    }
}
