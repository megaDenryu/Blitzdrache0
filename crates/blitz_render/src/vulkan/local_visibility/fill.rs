//! 2枚の画像を遮蔽なしの符号値で埋め、フレームの境で休むレイアウト(GENERAL)へ移す局面。
//! 呼ばれるのは画像を確保した直後だけであり、生成のときと画面寸法へ追従したときの両方が同じこの工程を通る。
//!
//! ぼかし後の画像を埋めることが、局所可視性補正を持たない世界の退避そのものである。パスを1本も積まない世界では
//! この値が最後まで残り、8ビット無符号正規化の符号値255はちょうど1.0であるため拡散間接照度を1ビットも変えない。
//!
//! 生の側も同じ値で埋めるのは、休むレイアウトをGENERALだと宣言することを2枚で揃えるためである。
//! 埋めずに残すと、局所可視性補正を積む最初のフレームがUNDEFINEDの画像へGENERALからの遷移を掛けることになる。

use ash::vk;

use super::images::局所可視度の画像組;
use crate::error::レンダラーエラー;
use crate::vulkan::transfer::転送実行環境;

/// 遮蔽なしを表す符号値255を8ビット無符号正規化で表した値。
const 遮蔽なしの色: vk::ClearColorValue = vk::ClearColorValue {
    float32: [1.0, 1.0, 1.0, 1.0],
};

fn 部分範囲() -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1)
}

/// 前提: 2枚とも確保直後でレイアウトはUNDEFINEDであり、GPUはまだどちらも使っていない。
pub(super) fn 遮蔽なしで埋める(
    device: &ash::Device,
    転送環境: &転送実行環境,
    画像組: &局所可視度の画像組,
) -> Result<(), レンダラーエラー> {
    let 画像一覧 = [画像組.生.画像, 画像組.ぼかし後.画像];
    転送環境.一括実行する(device, |command_buffer| {
        for 画像 in 画像一覧 {
            汎用へ遷移する(device, command_buffer, 画像);
        }
        for 画像 in 画像一覧 {
            // 安全性: command_bufferは記録中で、画像は直前のバリアでGENERALへ移っている。
            unsafe {
                device.cmd_clear_color_image(command_buffer, 画像, vk::ImageLayout::GENERAL, &遮蔽なしの色, &[部分範囲()]);
            }
        }
    })
}

/// 消去の宛先にも記憶画像の書き込みにも使えるGENERALへ移す。以降このレイアウトから二度と動かさない。
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
