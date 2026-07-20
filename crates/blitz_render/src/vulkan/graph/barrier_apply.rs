//! バリア導出結果(画像バリア記述)を実際のVulkan呼び出しへ変換する。
//! バリア発行の呼び出しは、グラフ実行器のこの1箇所に集約する
//! （参照: `_doc/設計/レンダーグラフ.md`「M5のDoD対応」）。

use ash::vk;

use super::barrier_derivation::画像バリア記述;
use super::registry::画像レジストリ;

/// バリア一覧を1回の`cmd_pipeline_barrier2`で発行する。空なら何もしない。
pub(crate) fn 発行する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    レジストリ: &画像レジストリ,
    バリア一覧: &[画像バリア記述],
) {
    if バリア一覧.is_empty() {
        return;
    }
    let vkバリア一覧: Vec<vk::ImageMemoryBarrier2> = バリア一覧
        .iter()
        .map(|バリア| 変換する(レジストリ, バリア))
        .collect();
    let 依存情報 = vk::DependencyInfo::default().image_memory_barriers(&vkバリア一覧);
    // 安全性: command_bufferは記録中で、各画像はレジストリに登録済みのVulkan画像。
    unsafe { device.cmd_pipeline_barrier2(command_buffer, &依存情報) };
}

fn 変換する<'a>(
    レジストリ: &'a 画像レジストリ,
    バリア: &'a 画像バリア記述,
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
