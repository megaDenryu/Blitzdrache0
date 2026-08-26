//! 呼び出し元が1つの単位として追加・解除する描画対象のGPU資源と、その描画対象だけを結ぶディスクリプタセットの束。
//! 束専用のディスクリプタプールを持つため、束の解除はプール1つの破棄で完結し、ディスクリプタセット添字はこの束の内側に閉じる。
//! 注意: 添字が束の内側で閉じることが、他の束の追加・解除で描画対象添字がずれないことの根拠である。
//! 個体が1体だけの描画対象が読む可視ID列(`単一個体用可視id列`)も、対象ごとに確保せずこの束が1つだけ持って共有する。
//! 読込時の確保は`create`、材質スロットを選べばセットが決まる状態は`descriptor_pick`にある。

mod create;
mod descriptor_pick;

use super::render_object_resources::描画対象資源;
use crate::draw_bundle_id::描画束ID;
use crate::vulkan::descriptor::描画対象ディスクリプタプール;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::visible_id::可視ID列バッファ;

pub(in crate::renderer::scene_draw_resources) use descriptor_pick::対象のディスクリプタ選択;

pub(super) struct チャンク描画資源 {
    id: 描画束ID, // 呼び出し元が与えた識別子。解除するときはこの値で対象を特定する
    影方針: crate::描画束の影方針,
    描画対象資源一覧: Vec<描画対象資源>,
    単一個体用可視id列: 可視ID列バッファ, // 値0だけの可視ID列。内容は生成後に変えない
    ディスクリプタ: 描画対象ディスクリプタプール,
}

impl チャンク描画資源 {
    pub(super) fn id(&self) -> 描画束ID {
        self.id
    }

    pub(super) fn 描画対象数(&self) -> usize {
        self.描画対象資源一覧.len()
    }

    pub(super) fn 影方針(&self) -> crate::描画束の影方針 {
        self.影方針
    }

    /// 描画発行の作業領域を積むための走査。束の中での描画対象添字と、描画対象資源と、材質スロットを選べばセットが決まる状態を組で返す。
    /// 添字を返すのは、そのフレームの可視個体選択が束IDと描画対象添字の対で引かれるためである。
    pub(super) fn 描画対象と対応セット(
        &self,
        フレーム添字: フレームスロット添字,
    ) -> impl Iterator<Item = (usize, &描画対象資源, 対象のディスクリプタ選択<'_>)> {
        self.描画対象資源一覧
            .iter()
            .enumerate()
            .map(move |(添字, 資源)| (添字, 資源, 対象のディスクリプタ選択::生成する(&self.ディスクリプタ, 添字, フレーム添字)))
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
