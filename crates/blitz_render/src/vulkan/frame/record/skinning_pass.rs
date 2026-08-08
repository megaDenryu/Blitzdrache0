//! GPUスキニングパスの宣言(判断44): コンピュートでスキン済み頂点バッファを書く。
//! GPU命令をコマンドバッファへ積むクロージャはバインド+プッシュ定数(頂点数)+計算の発行のみ。

use ash::vk;

use crate::vulkan::frame::スキニング描画入力;
use crate::vulkan::graph::{バッファハンドル, バッファ用途, パス宣言, パス種別};

pub(super) fn 作る<'a>(出力ハンドル: バッファハンドル, 入力: &'a スキニング描画入力) -> パス宣言<'a> {
    パス宣言::生成する(
        "スキニング",
        Vec::new(),
        Vec::new(),
        Vec::new(),
        vec![(出力ハンドル, バッファ用途::コンピュート書き)],
        パス種別::コンピュート,
        move |文脈| {
            // 宣言(書き:コンピュート書き)が実際の使用と一致することを検査する(判断28: 宣言=真実)。
            let _ = 文脈.バッファを解決する(出力ハンドル);

            let device = 文脈.device();
            let command_buffer = 文脈.コマンドバッファ();
            let set一覧 = [入力.ディスクリプタセット];
            let 頂点数バイト列 = 入力.頂点数.to_le_bytes();
            let 計算の班数 = 入力.頂点数.div_ceil(64);

            // 安全性: command_bufferは記録中で、pipeline・ディスクリプタセットは生成済み。
            // プッシュ定数の範囲(COMPUTE・4バイト)はパイプラインlayoutの宣言と一致する。
            unsafe {
                device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, 入力.pipeline);
                device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::COMPUTE, 入力.layout, 0, &set一覧, &[]);
                device.cmd_push_constants(command_buffer, 入力.layout, vk::ShaderStageFlags::COMPUTE, 0, &頂点数バイト列);
                device.cmd_dispatch(command_buffer, 計算の班数, 1, 1);
            }
        },
    )
}
