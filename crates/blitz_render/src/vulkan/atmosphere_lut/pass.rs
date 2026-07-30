//! 大気のベイク済み画像生成パスの宣言。コンピュートでベイク済み画像のストレージ画像を焼く。
//! 本番のフレーム記録と読み戻し検査の両方がこの1つの宣言を使うため、検査は本番と同じバリア導出を通る。
//! begin/end renderingは無く、記録クロージャはバインドとディスパッチだけを書く(粒子更新パスと同じ様式)。
//!
//! 書き込み先を「書き画像」として宣言することで、後続パスがこのベイク済み画像を読むときのバリアをグラフが導く。
//! パス名はGPU計器の区間名としてそのまま`--report-gpu-times`の表に現れる。

use ash::vk;

use super::大気のベイク済み画像の生成入力;
use crate::vulkan::graph::{パス宣言, パス種別, 画像ハンドル, 画像用途};

pub(crate) fn 作る<'a>(
    パス名: &'static str,
    書き込み先: 画像ハンドル,
    読み画像: Vec<(画像ハンドル, 画像用途)>,
    入力: &'a 大気のベイク済み画像の生成入力,
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
            let [横のワークグループ数, 縦のワークグループ数, 奥のワークグループ数] = 入力.ワークグループ数;

            // 安全性: command_bufferは記録中で、pipeline・ディスクリプタセットは生成済み。
            unsafe {
                device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, 入力.pipeline);
                device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::COMPUTE, 入力.layout, 0, &set一覧, &[]);
            }
            押し込む(device, command_buffer, 入力);
            // 安全性: 直前にパイプラインとディスクリプタを束縛済みで、ワークグループ数は寸法から切り上げた正当な値。
            unsafe {
                device.cmd_dispatch(command_buffer, 横のワークグループ数, 縦のワークグループ数, 奥のワークグループ数);
            }
        },
    )
}

/// 押し込み定数を持つパスだけがディスパッチ前に書く。持たないパスのレイアウトには範囲が無く、書けば
/// validationが範囲外として指摘するため、空のバイト列では発行しない。
fn 押し込む(device: &ash::Device, command_buffer: vk::CommandBuffer, 入力: &大気のベイク済み画像の生成入力) {
    let バイト列 = 入力.押し込み定数.バイト列();
    if バイト列.is_empty() {
        return;
    }
    // 安全性: command_bufferは記録中で、layoutはこの入力のパイプラインのものである。
    // バイト列の長さはパイプラインレイアウトが宣言した押し込み定数の範囲と、シェーダー側の宣言の両方に一致する。
    unsafe {
        device.cmd_push_constants(command_buffer, 入力.layout, vk::ShaderStageFlags::COMPUTE, 0, &バイト列);
    }
}
