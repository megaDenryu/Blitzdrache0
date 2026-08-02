//! 描画対象1つぶんの、材質スロットを選べばディスクリプタセットが決まる状態。担当するのは、束が持つプールと
//! 描画対象添字とフレームスロット添字の3つを組にして運び、材質スロット添字だけを後から受け取ることである。
//!
//! 束の走査の時点でセットを1つに決められないのは、どの材質を束ねるかがプリミティブ描画発行ごとに変わるためである。
//! 3つを別々に持ち回すと、描画発行を積む側が束と対象の対応を組み立て直すことになる。

use ash::vk;

use crate::renderer::scene_draw_resources::bundle_material_reference::束内材質参照;
use crate::vulkan::descriptor::描画対象ディスクリプタプール;
use crate::vulkan::sync::フレームスロット添字;

#[derive(Clone, Copy)]
pub(in crate::renderer::scene_draw_resources) struct 対象のディスクリプタ選択<'a> {
    プール: &'a 描画対象ディスクリプタプール,
    描画対象添字: usize,
    フレーム添字: フレームスロット添字,
}

impl<'a> 対象のディスクリプタ選択<'a> {
    pub(super) fn 生成する(
        プール: &'a 描画対象ディスクリプタプール, 描画対象添字: usize, フレーム添字: フレームスロット添字
    ) -> Self {
        Self {
            プール,
            描画対象添字,
            フレーム添字,
        }
    }

    /// その材質を束ねるディスクリプタセット。参照はこの対象の材質スロット資源が材質スロット番号から解決した値である。
    pub(in crate::renderer::scene_draw_resources) fn スロット(self, 材質参照: 束内材質参照) -> vk::DescriptorSet {
        self.プール.set(self.描画対象添字, 材質参照.セットの並びの添字(), self.フレーム添字)
    }
}
