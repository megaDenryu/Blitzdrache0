//! グラフ実行器のGPUタイムスタンプ書き込み補助(判断30)。パス添字→クエリ開始添字の
//! 決定と、実際の`vkCmdWriteTimestamp2`発行を実行器から切り出す。

use ash::vk;

use crate::vulkan::command_sink::GPU命令の積み先;
use crate::vulkan::gpu_timing::パス数上限;

/// パス添字(0起点)からこのパスのクエリ開始添字(前後2発ぶんの前半)を求める。
/// `パス数上限`を超えるパス数は実装のバグとしてpanicする(クエリプール容量は
/// `パス数上限`ぶんしか無いため)。
pub(super) fn クエリ開始添字を求める(パス添字: usize) -> u32 {
    let パス添字u32 = u32::try_from(パス添字).unwrap_or_else(|_| panic!("パス添字がu32に収まらない: {パス添字}"));
    if パス添字u32 >= パス数上限 {
        panic!("1フレームのパス数が上限({パス数上限})を超えた: {パス添字u32}");
    }
    パス添字u32 * 2
}

pub(super) fn タイムスタンプを書く(積み先: GPU命令の積み先<'_>, pool: vk::QueryPool, クエリ添字: u32) {
    // 安全性: command_bufferは記録中で、poolはフレーム記録の積み始めでリセット済み
    // (`frame::session::積み始める`)。ALL_COMMANDS基準はTOP_OF_PIPE相当を
    // 避けるため(参照: `_doc/設計/レンダーグラフ.md`「GPU計測」)。
    unsafe {
        積み先
            .論理デバイス()
            .cmd_write_timestamp2(積み先.コマンドバッファ(), vk::PipelineStageFlags2::ALL_COMMANDS, pool, クエリ添字)
    };
}
