//! 呼び出し元が1つの単位として追加・解除する描画対象のGPU資源と、その描画対象だけを結ぶディスクリプタセットの束。
//! 束専用のディスクリプタプールを持つため、束の解除はプール1つの破棄で完結し、ディスクリプタセット添字はこの束の内側に閉じる。
//! 注意: 添字が束の内側で閉じることが、他の束の追加・解除で描画対象添字がずれないことの根拠である。
//! 個体が1体だけの描画対象が読む可視ID列も、対象ごとに確保せずこの束が1つだけ持って共有する。
//! 読込時の確保は`create`にある。

mod create;

use ash::vk;

use super::render_object_resources::描画対象資源;
use crate::draw_bundle_id::描画束ID;
use crate::vulkan::descriptor::描画対象ディスクリプタプール;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::visible_id::可視ID列バッファ;

pub(super) struct チャンク描画資源 {
    /// 呼び出し元が与えた識別子。解除するときはこの値で対象を特定する。
    id: 描画束ID,
    描画対象資源一覧: Vec<描画対象資源>,
    /// 値0だけの可視ID列。個体が1体だけの対象がすべてここを読むため、内容は生成後に変えない。
    単一個体用可視id列: 可視ID列バッファ,
    ディスクリプタ: 描画対象ディスクリプタプール,
}

impl チャンク描画資源 {
    pub(super) fn id(&self) -> 描画束ID {
        self.id
    }

    pub(super) fn 描画対象数(&self) -> usize {
        self.描画対象資源一覧.len()
    }

    /// 描画発行の作業領域を積むための走査。束の中での描画対象添字と、描画対象資源と、そのフレームで束縛すべきディスクリプタセットを組で返す。
    /// 添字を返すのは、そのフレームの可視個体選択が束IDと描画対象添字の対で引かれるためである。
    pub(super) fn 描画対象と対応セット(
        &self,
        フレーム添字: フレームスロット添字,
    ) -> impl Iterator<Item = (usize, &描画対象資源, vk::DescriptorSet)> {
        self.描画対象資源一覧
            .iter()
            .enumerate()
            .map(move |(添字, 資源)| (添字, 資源, self.ディスクリプタ.set(添字, フレーム添字)))
    }

    /// 注意: ディスクリプタセットが指すテクスチャとシェーダー定数より先にディスクリプタプールを破棄する。
    /// 前提: 呼び出し元がGPU側の使用完了を待ってから呼ぶ。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.ディスクリプタ.破棄する(device);
        self.単一個体用可視id列.破棄する(device);
        資源一覧を破棄する(device, &self.描画対象資源一覧);
    }
}

fn 資源一覧を破棄する(device: &GPUデバイス, 描画対象資源一覧: &[描画対象資源]) {
    for 資源 in 描画対象資源一覧 {
        資源.破棄する(device);
    }
}
