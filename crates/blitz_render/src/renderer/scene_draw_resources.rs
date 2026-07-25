//! 描画対象の個数に連動するGPU資源の束: 所有チャンクごとの描画資源の集合・全チャンクが共有するディスクリプタセットレイアウト・毎フレームの描画入力作業領域2本。
//! 作業領域は毎フレーム空にしてからチャンク一覧を先頭から走査して積み直すため、2本の要素数は常に全チャンクの描画対象数の合計と一致する。
//! ディスクリプタセット添字はチャンクの内側で閉じるため、あるチャンクの追加・解除が他のチャンクの添字をずらさない。
//! 生成は`create`、チャンク1つぶんの資源は`chunk_draw_resources`、毎フレームの作業領域更新は`work_area`、描画対象1つぶんの資源は`render_object_resources`にある。

mod chunk_draw_resources;
mod create;
mod render_object_resources;
mod work_area;

use ash::vk;

use crate::vulkan::descriptor::ディスクリプタレイアウト;
use crate::vulkan::frame::{シャドウ描画入力, ジオメトリ入力};
use crate::vulkan::tracked_device::GPUデバイス;
use chunk_draw_resources::チャンク描画資源;

pub(super) use create::シーン描画資源生成要求;
pub(super) use work_area::作業領域更新入力;

pub(super) struct シーン描画資源 {
    /// 全チャンクの描画対象が同じ内容のセットを使うため、チャンクごとに作らずここが1つだけ所有する。
    レイアウト: ディスクリプタレイアウト,
    チャンク一覧: Vec<チャンク描画資源>,
    ジオメトリ入力作業領域: Vec<ジオメトリ入力>,
    シャドウ入力作業領域: Vec<シャドウ描画入力>,
}

impl シーン描画資源 {
    /// シーンパイプラインとシャドウパイプラインの生成が必要とするディスクリプタセットレイアウト。
    /// シーンを差し替えても互換レイアウトを作り直すだけのため、パイプラインは作り直さない。
    pub(super) fn ディスクリプタlayout(&self) -> vk::DescriptorSetLayout {
        self.レイアウト.handle()
    }

    /// 注意: 各チャンクがディスクリプタプールを破棄し終えてからレイアウトを破棄する(レイアウトはプールが割り当てたセットの生存前提である)。
    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この束はその1段として呼ばれる(GPU待機済み)。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        for チャンク in &self.チャンク一覧 {
            チャンク.破棄する(device);
        }
        self.レイアウト.破棄する(device);
    }
}
