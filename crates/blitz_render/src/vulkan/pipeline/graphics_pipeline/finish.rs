//! `create_graphics_pipelines`の生成結果からパイプラインを取り出す。
//! レイアウトの後始末をここでしないのは、レイアウトの所有者がパイプラインの生成を頼んだ側だからである。

use ash::vk;

use crate::error::レンダラーエラー;

pub(in crate::vulkan::pipeline) fn パイプラインを取り出す(
    生成結果: Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::Result)>,
) -> Result<vk::Pipeline, レンダラーエラー> {
    match 生成結果 {
        Ok(一覧) => {
            let Some(&handle) = 一覧.first() else {
                // create_graphics_pipelinesが成功を返したのにパイプラインが0本なのは
                // Vulkan実装がその契約を破っている状態であり回復不能。
                panic!("create_graphics_pipelinesが成功したのにパイプラインが0本だった");
            };
            Ok(handle)
        }
        Err((_, 誤り)) => Err(誤り.into()),
    }
}
