//! バリア導出結果(画像バリア記述・バッファバリア記述)を実際のVulkan呼び出しへ変換する。
//! バリア発行の呼び出しは、グラフ実行器のこの1箇所に集約する
//! （参照: `_doc/設計/レンダーグラフ.md`「M5のDoD対応」）。

use ash::vk;

use super::barrier_derivation::画像バリア記述;
use super::buffer_barrier_derivation::バッファバリア記述;
use super::buffer_registry::バッファレジストリ;
use super::registry::画像レジストリ;
use crate::vulkan::command_sink::GPU命令の積み先;

pub(super) struct バリア発行器<'a> {
    積み先: GPU命令の積み先<'a>,
    画像レジストリ: &'a 画像レジストリ,
    バッファレジストリ: &'a バッファレジストリ,
    vk画像バリア一覧: Vec<vk::ImageMemoryBarrier2<'static>>,
    vkバッファバリア一覧: Vec<vk::BufferMemoryBarrier2<'static>>,
}

impl<'a> バリア発行器<'a> {
    pub(super) fn 生成する(積み先: GPU命令の積み先<'a>, 画像レジストリ: &'a 画像レジストリ, バッファレジストリ: &'a バッファレジストリ) -> Self {
        Self {
            積み先,
            画像レジストリ,
            バッファレジストリ,
            vk画像バリア一覧: Vec::new(),
            vkバッファバリア一覧: Vec::new(),
        }
    }

    // 画像・バッファのバリアを1回の命令で発行する。一覧の容量は同じグラフのパス間で再利用する。
    pub(super) fn バリアを発行する(&mut self, 画像バリア一覧: &[画像バリア記述], バッファバリア一覧: &[バッファバリア記述]) {
        self.vk画像バリア一覧.clear();
        self.vkバッファバリア一覧.clear();
        if 画像バリア一覧.is_empty() && バッファバリア一覧.is_empty() {
            return;
        }
        self.vk画像バリア一覧.extend(画像バリア一覧.iter().map(|バリア| 画像バリアへ変換する(self.画像レジストリ, バリア)));
        self.vkバッファバリア一覧.extend(バッファバリア一覧.iter().map(|バリア| バッファバリアへ変換する(self.バッファレジストリ, バリア)));
        let 依存情報 = vk::DependencyInfo::default().image_memory_barriers(&self.vk画像バリア一覧).buffer_memory_barriers(&self.vkバッファバリア一覧);
        // 安全性: command_bufferは記録中で、各画像・バッファはレジストリに登録済みのVulkanリソース。一覧は命令の発行が返るまで有効である。
        unsafe { self.積み先.論理デバイス().cmd_pipeline_barrier2(self.積み先.コマンドバッファ(), &依存情報) };
    }
}

// 生成する記述はハンドルと数値だけを持ち、p_nextも空であるため、入力を借用しない。
fn 画像バリアへ変換する(レジストリ: &画像レジストリ, バリア: &画像バリア記述) -> vk::ImageMemoryBarrier2<'static> {
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

fn バッファバリアへ変換する(レジストリ: &バッファレジストリ, バリア: &バッファバリア記述) -> vk::BufferMemoryBarrier2<'static> {
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
