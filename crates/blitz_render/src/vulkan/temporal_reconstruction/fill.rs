//! 履歴画像2枚を零で埋め、フレームの境で休むレイアウト(GENERAL)へ移す局面。呼ばれるのは画像を確保した直後だけであり、
//! 生成のときと画面寸法へ追従したときの両方が同じこの工程を通る。
//!
//! 埋めるのは、確保したばかりの画像の中身が不定だからである。半精度の不定ビット列は非数にもなりうるため、
//! 最初のフレームがそれを混ぜると以後どれだけ混ぜても非数が消えない。
//!
//! 動きベクトル画像を埋めないのは、シーン描画のパスが毎フレーム消去してから書くためである。前のフレームの内容を
//! 1画素も引き継がないため、確保直後の中身が結果へ現れる経路が無い。

use ash::vk;

use super::images::時間再構成の画像組;
use crate::error::レンダラーエラー;
use crate::vulkan::transfer::転送実行環境;

fn 部分範囲() -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1)
}

/// 前提: 2枚とも確保直後でレイアウトはUNDEFINEDであり、GPUはまだどちらも使っていない。
pub(super) fn 履歴を零で埋める(
    device: &ash::Device,
    転送環境: &転送実行環境,
    画像組: &時間再構成の画像組,
) -> Result<(), レンダラーエラー> {
    let 色 = vk::ClearColorValue { float32: [0.0; 4] };
    let 画像一覧: Vec<vk::Image> = 画像組.履歴.iter().map(|履歴| 履歴.画像).collect();
    転送環境.一括実行する(device, |command_buffer| {
        for 画像 in &画像一覧 {
            汎用へ遷移する(device, command_buffer, *画像);
        }
        for 画像 in &画像一覧 {
            // 安全性: command_bufferは記録中で、画像は直前のバリアでGENERALへ移っている。
            unsafe {
                device.cmd_clear_color_image(command_buffer, *画像, vk::ImageLayout::GENERAL, &色, &[部分範囲()]);
            }
        }
    })
}

/// 消去の宛先にもカラー添付の書き込みにも標本にも使えるGENERALへ移す。以降このレイアウトから二度と動かさない。
/// フレームをまたいで中身を保つ画像であるため、レイアウトを毎フレーム往復させると保つべき内容の根拠が1箇所に無くなる。
fn 汎用へ遷移する(device: &ash::Device, command_buffer: vk::CommandBuffer, 画像: vk::Image) {
    let バリア = vk::ImageMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::TOP_OF_PIPE)
        .dst_stage_mask(vk::PipelineStageFlags2::CLEAR)
        .dst_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
        .old_layout(vk::ImageLayout::UNDEFINED)
        .new_layout(vk::ImageLayout::GENERAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(画像)
        .subresource_range(部分範囲());
    let バリア一覧 = [バリア];
    let 依存情報 = vk::DependencyInfo::default().image_memory_barriers(&バリア一覧);
    // 安全性: command_bufferは転送実行環境が記録用に開始済みで、画像は生成直後のUNDEFINEDレイアウト。このバリアが唯一の書き手。
    unsafe { device.cmd_pipeline_barrier2(command_buffer, &依存情報) };
}
