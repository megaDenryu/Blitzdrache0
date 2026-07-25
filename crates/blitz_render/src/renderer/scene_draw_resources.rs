//! 描画対象の個数に連動するGPU資源の束: 描画対象資源一覧・ディスクリプタ一式・毎フレームの描画入力作業領域2本。
//! ディスクリプタセットは描画対象数×フレームインフライト数で確保して`描画対象添字 * フレームインフライト数 + フレーム添字`で引き、
//! 作業領域は毎フレーム空にしてから描画対象資源一覧を走査して積み直すため、4つの要素数はすべて描画対象数という1つの値に従う。
//! 個数がずれるとディスクリプタセット添字が範囲外になるため、生成・差し替え・破棄をこの型に閉じて個数を決める箇所を1つにする。
//! 生成は`create`、毎フレームの作業領域更新は`work_area`、描画対象1つぶんの資源は`render_object_resources`にある。

mod create;
mod render_object_resources;
mod work_area;

use ash::vk;

use crate::vulkan::descriptor::ディスクリプタ一式;
use crate::vulkan::frame::{シャドウ描画入力, ジオメトリ入力};
use crate::vulkan::tracked_device::GPUデバイス;
use render_object_resources::描画対象資源;

pub(super) use create::シーン描画資源生成要求;
pub(super) use work_area::作業領域更新入力;

pub(super) struct シーン描画資源 {
    描画対象資源一覧: Vec<描画対象資源>,
    ディスクリプタ: ディスクリプタ一式,
    ジオメトリ入力作業領域: Vec<ジオメトリ入力>,
    シャドウ入力作業領域: Vec<シャドウ描画入力>,
}

impl シーン描画資源 {
    /// シーンパイプラインとシャドウパイプラインの生成が必要とするディスクリプタセットレイアウト。
    /// シーンを差し替えても互換レイアウトを作り直すだけのため、パイプラインは作り直さない。
    pub(super) fn ディスクリプタlayout(&self) -> vk::DescriptorSetLayout {
        self.ディスクリプタ.layout
    }

    /// 注意: ディスクリプタセットが指すテクスチャとユニフォームより先にディスクリプタプールを破棄する。
    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この束はその1段として呼ばれる(GPU待機済み)。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.ディスクリプタ.破棄する(device);
        for 資源 in &self.描画対象資源一覧 {
            資源.破棄する(device);
        }
    }
}
