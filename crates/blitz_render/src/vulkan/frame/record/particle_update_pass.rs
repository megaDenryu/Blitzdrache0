//! 粒子更新パスの宣言(判断29): コンピュートで粒子ストレージバッファを書く。
//! begin/end renderingは無い(コンピュートパス、GPU命令をコマンドバッファへ積むクロージャはバインド+計算の発行のみ)。

use ash::vk;

use crate::vulkan::frame::粒子描画入力;
use crate::vulkan::graph::{バッファハンドル, バッファ用途, パス宣言, パス種別};

pub(super) fn 粒子更新パスを宣言する<'a>(
    粒子ハンドル: バッファハンドル, 粒子入力: &'a 粒子描画入力
) -> パス宣言<'a> {
    パス宣言::生成する(
        "粒子更新",
        Vec::new(),
        Vec::new(),
        Vec::new(),
        vec![(粒子ハンドル, バッファ用途::コンピュート書き)],
        パス種別::コンピュート,
        move |文脈| {
            // 宣言(書き:コンピュート書き)が実際の使用と一致することを検査する
            // (判断28: 宣言=真実。戻り値のvk::Buffer自体はディスクリプタセット経由の
            // 束縛で既に解決済みのため使わない)。
            let _ = 文脈.バッファを解決する(粒子ハンドル);

            let device = 文脈.device();
            let command_buffer = 文脈.コマンドバッファ();
            let set一覧 = [粒子入力.ディスクリプタセット];
            let 計算の班数 = 粒子入力.更新スレッド数.div_ceil(64);

            // 安全性: command_bufferは記録中で、pipeline・ディスクリプタセットは生成済み。
            unsafe {
                device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, 粒子入力.コンピュートパイプライン);
                device.cmd_bind_descriptor_sets(
                    command_buffer,
                    vk::PipelineBindPoint::COMPUTE,
                    粒子入力.コンピュートlayout,
                    0,
                    &set一覧,
                    &[],
                );
                device.cmd_dispatch(command_buffer, 計算の班数, 1, 1);
            }
        },
    )
}
