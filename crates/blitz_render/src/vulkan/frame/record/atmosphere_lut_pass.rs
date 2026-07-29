//! 大気LUT生成パスの宣言。コンピュートでLUTのストレージ画像を焼く。
//! begin/end renderingは無く、記録クロージャはバインドとディスパッチだけを書く(粒子更新パスと同じ様式)。
//!
//! 書き込み先を「書き画像」として宣言することで、後続パスがこのLUTを読むときのバリアをグラフが導く。
//! パス名はGPU計器の区間名としてそのまま`--report-gpu-times`の表に現れる。

use ash::vk;

use crate::vulkan::atmosphere_lut::大気LUT生成入力;
use crate::vulkan::graph::{パス宣言, パス種別, 画像ハンドル, 画像用途};

pub(super) fn 作る<'a>(
    パス名: &'static str,
    書き込み先: 画像ハンドル,
    読み画像: Vec<(画像ハンドル, 画像用途)>,
    入力: &'a 大気LUT生成入力,
) -> パス宣言<'a> {
    パス宣言::生成する(
        パス名,
        読み画像,
        vec![(書き込み先, 画像用途::コンピュート書き)],
        Vec::new(),
        Vec::new(),
        パス種別::コンピュート,
        move |文脈| {
            // 宣言(書き:コンピュート書き)が実際の使用と一致することを検査する(判断28: 宣言=真実)。
            // 戻り値のvk::Image自体はディスクリプタセット経由の束縛で既に解決済みのため使わない。
            let _ = 文脈.画像を解決する(書き込み先);

            let device = 文脈.device();
            let command_buffer = 文脈.コマンドバッファ();
            let set一覧 = [入力.ディスクリプタセット];
            let [横のワークグループ数, 縦のワークグループ数] = 入力.ワークグループ数;

            // 安全性: command_bufferは記録中で、pipeline・ディスクリプタセットは生成済み。
            unsafe {
                device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, 入力.pipeline);
                device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::COMPUTE, 入力.layout, 0, &set一覧, &[]);
                device.cmd_dispatch(command_buffer, 横のワークグループ数, 縦のワークグループ数, 1);
            }
        },
    )
}
