//! 1つの布コンピュートパスをレンダーグラフへ登録する。

use ash::vk;

use crate::vulkan::frame::布描画入力;
use crate::vulkan::graph::{グラフ, バッファハンドル, バッファ用途, パス宣言, パス種別};

#[allow(clippy::too_many_arguments)]
pub(super) fn 積む<'a>(
    グラフ: &mut グラフ<'a>,
    名前: &'static str,
    読み: Vec<(バッファハンドル, バッファ用途)>,
    書き: Vec<(バッファハンドル, バッファ用途)>,
    入力: &'a 布描画入力,
    pipeline: vk::Pipeline,
    スレッド数: u32,
) {
    let セット一覧 = [入力.ディスクリプタセット];
    let layout = 入力.layout;
    グラフ.パスを積む(パス宣言::生成する(
        名前,
        Vec::new(),
        Vec::new(),
        読み,
        書き,
        パス種別::コンピュート,
        move |文脈| {
            let device = 文脈.積み先().論理デバイス();
            let command_buffer = 文脈.積み先().コマンドバッファ();
            // 安全性: command_bufferは記録中で、pipeline・ディスクリプタセットは生成済み。
            unsafe {
                device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, pipeline);
                device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::COMPUTE, layout, 0, &セット一覧, &[]);
                device.cmd_dispatch(command_buffer, スレッド数.div_ceil(64), 1, 1);
            }
        },
    ));
}
