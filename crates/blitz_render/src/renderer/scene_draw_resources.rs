//! 描画対象の個数に連動するGPU資源の束: 呼び出し元が単位として扱う束の集合・全束が共有するディスクリプタセットレイアウト・毎フレームの描画入力作業領域2本・解除予約された束の待機列。
//! 作業領域は毎フレーム空にしてから束一覧を先頭から走査して積み直すため、2本の要素数は常に全束の描画対象数の合計と一致する。
//! ディスクリプタセット添字は束の内側で閉じるため、ある束の追加・解除が他の束の添字をずらさない。
//! 生成は`create`、束1つぶんの資源は`chunk_draw_resources`、束の追加と解除は`bundle_lifecycle`、毎フレームの作業領域更新は`work_area`、描画対象1つぶんの資源は`render_object_resources`にある。

mod bundle_lifecycle;
mod chunk_draw_resources;
mod create;
mod render_object_resources;
mod work_area;

use ash::vk;

use crate::draw_bundle_id::描画束ID;
use crate::vulkan::descriptor::ディスクリプタレイアウト;
use crate::vulkan::frame::{シャドウ描画入力, ジオメトリ入力};
use crate::vulkan::tracked_device::GPUデバイス;
use bundle_lifecycle::破棄待ち束;
use chunk_draw_resources::チャンク描画資源;

pub(super) use create::{シーン描画資源生成要求, 束追加材料};
pub(super) use work_area::作業領域更新入力;

pub(super) struct シーン描画資源 {
    /// 全束の描画対象が同じ内容のセットを使うため、束ごとに作らずここが1つだけ所有する。
    レイアウト: ディスクリプタレイアウト,
    チャンク一覧: Vec<チャンク描画資源>,
    ジオメトリ入力作業領域: Vec<ジオメトリ入力>,
    シャドウ入力作業領域: Vec<シャドウ描画入力>,
    /// 解除を予約された束。GPUが使い終わるまでフレームを数えてから破棄する(規律は`bundle_lifecycle`が持つ)。
    破棄待ち: Vec<破棄待ち束>,
    /// 実際にGPU資源を破棄し終えた束のID。呼び出し元が引き取るまで貯まる。
    /// 解除の予約と実破棄は時点が違い、資源の会計は実破棄の時点でしか減らせないため、破棄の事実をIDで外へ返す。
    実破棄済みid一覧: Vec<描画束ID>,
}

impl シーン描画資源 {
    /// シーンパイプラインとシャドウパイプラインの生成が必要とするディスクリプタセットレイアウト。
    /// シーンを差し替えても互換レイアウトを作り直すだけのため、パイプラインは作り直さない。
    pub(super) fn ディスクリプタlayout(&self) -> vk::DescriptorSetLayout {
        self.レイアウト.handle()
    }

    /// 注意: 各束がディスクリプタプールを破棄し終えてからレイアウトを破棄する(レイアウトはプールが割り当てたセットの生存前提である)。
    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この束はその1段として呼ばれる(GPU待機済み)。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        for 待ち in &self.破棄待ち {
            待ち.束().破棄する(device);
        }
        for チャンク in &self.チャンク一覧 {
            チャンク.破棄する(device);
        }
        self.レイアウト.破棄する(device);
    }
}
