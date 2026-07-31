//! 布専用シャドウ経路のディスクリプタの所有者。フレームシェーダー定数1本だけを結ぶレイアウトと、
//! フレームスロットごとの2セットを保持する。触れるのは自分が作ったレイアウト・プール・セットだけである。
//!
//! 不変条件として、セットは生成時に一度書いたら以降書き換えない(結ぶバッファがスロットごとに固定のため)。
//! シーンのディスクリプタセットレイアウトを流用しないのは、布の影を描画対象の登録有無から切り離すためである。
//! 流用すると束縛できるセットが描画対象由来のものしかなく、束が0件のフレームで布の影が描けなくなる
//! (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「描画段階資源の器と布専用シャドウ経路」)。
//! 生成はレンダラー生成時の1回だけの局面であり、`create`が担う。

mod create;

use ash::vk;

use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};

/// フレームシェーダー定数のバインディング番号。shaders/cloth_shadow.slangの`[[vk::binding(3, 0)]]`と一致させる。
/// 3にそろえるのは、同じ内容を読むscene.slang・shadow.slangの宣言と番号を食い違わせないためである。
const フレームシェーダー定数のバインディング番号: u32 = 3;

pub(super) use create::生成する;

pub(super) struct 布シャドウディスクリプタ {
    layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    set一覧: [vk::DescriptorSet; 進行中フレーム数],
}

impl 布シャドウディスクリプタ {
    pub(super) fn layout(&self) -> vk::DescriptorSetLayout {
        self.layout
    }

    pub(super) fn set(&self, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        self.set一覧[フレーム添字.配列添字()]
    }

    /// 注意: プールの破棄がセットの解放を暗黙に行うため、プールをレイアウトより先に破棄する。
    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.layout, None);
        }
    }
}
