//! 立方体配列を生成直後に一度だけ深度読み取りのレイアウトへ移す局面。呼ばれるのはレンダラー生成時の1回だけである。
//!
//! これが要るのは、影を落とす灯を1件も持たない世界がこの画像へ1本もパスを積まないためである。パスが無ければ
//! レンダーグラフはバリアを1つも出さず、画像は生成直後の未定義のレイアウトのまま画素段のディスクリプタに束縛される。
//! 束縛された画像が宣言どおりのレイアウトに無い状態は検証層が指摘する(実際にvalidationで観測した)。
//!
//! 灯を持つ世界では最初のパスがレイアウトを移すため、この移動は「灯を1件も持たないフレームでも宣言が真である」ことを
//! 保証する側の仕事である。レンダーグラフが登録時に前提とする初期状態と同じレイアウトへ揃える
//! (参照: `crates/blitz_render/src/vulkan/graph/initial_state.rs`の前フレームシャドウマップ読み直後状態)。
//!
//! 中身は移さない。層の内容は未定義のままであり、影を落とす灯が現れた面だけが最初の記録で内容を持つ。
//! 内容が未定義の層を標本する経路は、局所光レコードの影の有無の枝が閉じている。

use ash::vk;

use super::{点光源の影の層数, 点光源の影の立方体配列};
use crate::error::レンダラーエラー;
use crate::vulkan::transfer::転送実行環境;

impl 点光源の影の立方体配列 {
    pub(crate) fn 初期レイアウトを深度読み取りへ整える(
        &self, 転送環境: &転送実行環境
    ) -> Result<(), レンダラーエラー> {
        let セッション = 転送環境.転送コマンドを積み始める()?;
        let 部分範囲 = vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::DEPTH)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(点光源の影の層数());
        let 障壁 = vk::ImageMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::NONE)
            .src_access_mask(vk::AccessFlags2::NONE)
            .dst_stage_mask(vk::PipelineStageFlags2::FRAGMENT_SHADER)
            .dst_access_mask(vk::AccessFlags2::SHADER_SAMPLED_READ)
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::DEPTH_READ_ONLY_OPTIMAL)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .image(self.画像)
            .subresource_range(部分範囲);
        let 障壁一覧 = [障壁];
        let 依存情報 = vk::DependencyInfo::default().image_memory_barriers(&障壁一覧);
        // 安全性: コマンドバッファは積み込み中であり、画像はbind_image_memory済みで他の誰も使っていない。
        let 積み先 = セッション.積み先();
        unsafe {
            積み先.論理デバイス().cmd_pipeline_barrier2(積み先.コマンドバッファ(), &依存情報);
        }
        セッション.送信して完了を待つ()
    }
}
