//! バリア導出結果(画像バリア記述・バッファバリア記述)を実際のVulkan呼び出しへ変換する。
//! バリア発行の呼び出しは、グラフ実行器のこの1箇所に集約する
//! （参照: `_doc/設計/レンダーグラフ.md`「M5のDoD対応」）。

use ash::vk;

use super::barrier_derivation::画像バリア記述;
use super::buffer_barrier_derivation::バッファバリア記述;
use super::buffer_registry::バッファレジストリ;
use super::registry::画像レジストリ;
use crate::vulkan::command_sink::GPU命令の積み先;

/// 画像・バッファのバリア一覧を1回の`cmd_pipeline_barrier2`で発行する。両方空なら何もしない。
pub(super) fn バリアを発行する(
    積み先: GPU命令の積み先<'_>,
    画像レジストリ: &画像レジストリ,
    バッファレジストリ: &バッファレジストリ,
    画像バリア一覧: &[画像バリア記述],
    バッファバリア一覧: &[バッファバリア記述],
) {
    if 画像バリア一覧.is_empty() && バッファバリア一覧.is_empty() {
        return;
    }
    let vk画像バリア一覧: Vec<vk::ImageMemoryBarrier2> = 画像バリア一覧.iter().map(|バリア| 画像バリアへ変換する(画像レジストリ, バリア)).collect();
    let vkバッファバリア一覧: Vec<vk::BufferMemoryBarrier2> = バッファバリア一覧
        .iter()
        .map(|バリア| バッファバリアへ変換する(バッファレジストリ, バリア))
        .collect();
    let 依存情報 = vk::DependencyInfo::default()
        .image_memory_barriers(&vk画像バリア一覧)
        .buffer_memory_barriers(&vkバッファバリア一覧);
    // 安全性: command_bufferは記録中で、各画像・バッファはレジストリに登録済みのVulkanリソース。
    unsafe { 積み先.論理デバイス().cmd_pipeline_barrier2(積み先.コマンドバッファ(), &依存情報) };
}

fn 画像バリアへ変換する<'a>(
    レジストリ: &'a 画像レジストリ, バリア: &'a 画像バリア記述
) -> vk::ImageMemoryBarrier2<'a> {
    let 部分範囲 = レジストリ.アスペクトを取得する(バリア.ハンドル).部分範囲();
    vk::ImageMemoryBarrier2::default()
        .src_stage_mask(バリア.前.stage)
        .src_access_mask(バリア.前.access)
        .dst_stage_mask(バリア.今.stage)
        .dst_access_mask(バリア.今.access)
        .old_layout(バリア.前.layout)
        .new_layout(バリア.今.layout)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(レジストリ.画像を取得する(バリア.ハンドル))
        .subresource_range(部分範囲)
}

fn バッファバリアへ変換する<'a>(
    レジストリ: &'a バッファレジストリ, バリア: &'a バッファバリア記述
) -> vk::BufferMemoryBarrier2<'a> {
    vk::BufferMemoryBarrier2::default()
        .src_stage_mask(バリア.前.stage)
        .src_access_mask(バリア.前.access)
        .dst_stage_mask(バリア.今.stage)
        .dst_access_mask(バリア.今.access)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .buffer(レジストリ.バッファを取得する(バリア.ハンドル))
        .offset(0)
        .size(vk::WHOLE_SIZE)
}
