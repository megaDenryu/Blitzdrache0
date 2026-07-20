//! `create_graphics_pipelines`の生成結果から粒子描画パイプラインを取り出す。
//! 失敗時はlayoutの後始末(破棄)まで行う(グラフィックスパイプラインの
//! 既存`finish`と同じ様式)。

use ash::vk;

use super::粒子描画パイプライン;
use crate::error::レンダラーエラー;

pub(super) fn パイプラインを取り出す(
    device: &ash::Device,
    layout: vk::PipelineLayout,
    生成結果: Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::Result)>,
) -> Result<粒子描画パイプライン, レンダラーエラー> {
    match 生成結果 {
        Ok(一覧) => {
            let Some(&handle) = 一覧.first() else {
                panic!("create_graphics_pipelinesが成功したのにパイプラインが0本だった");
            };
            Ok(粒子描画パイプライン { handle, layout })
        }
        Err((_, 誤り)) => {
            // 安全性: パイプライン生成に失敗したため、layoutを参照するパイプラインは存在しない。
            unsafe { device.destroy_pipeline_layout(layout, None) };
            Err(誤り.into())
        }
    }
}
