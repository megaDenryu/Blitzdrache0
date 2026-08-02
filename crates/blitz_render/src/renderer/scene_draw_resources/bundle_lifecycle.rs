//! 束の追加と解除。触れるのは`チャンク一覧`・`破棄待ち`・`実破棄済みid一覧`の3フィールドだけで、束がこの3つを移る規律をここが持つ。
//!
//! 解除された束をその場で破棄しないのは、過去のフレームで発行済みの描画コマンドがまだGPUで実行中でありうるためである。
//! 解除予約は描画発行より前に行うため、予約した時点でその束を使う新しい描画は発行されない。残るのは発行済みのものだけで、それらは進行中フレーム数ぶんのフレームが進めば必ず完了している(同じスロットを再び使うときにフェンスを待つため)。
//! したがって`進行中フレーム数`回の`破棄待ちを一フレーム進める`を経てから破棄する。
//! 同じ束IDの内容を入れ替える差し替えは`replace`にある。

mod replace;

use super::chunk_draw_resources::チャンク描画資源;
use super::create::束追加材料;
use super::シーン描画資源;
use crate::draw_bundle_id::描画束ID;
use crate::error::レンダラーエラー;
use crate::render_object_material::描画対象素材;
use crate::vulkan::sync::進行中フレーム数;
use crate::vulkan::tracked_device::GPUデバイス;

/// 解除予約された束と、破棄までに待つ残りフレーム数。
pub(super) struct 破棄待ち束 {
    束: チャンク描画資源,
    残りフレーム: usize,
}

impl 破棄待ち束 {
    pub(super) fn 束(&self) -> &チャンク描画資源 {
        &self.束
    }

    /// 残りを1減らし、破棄してよくなったかを返す。
    fn 一フレーム進める(&mut self) -> bool {
        self.残りフレーム = self.残りフレーム.saturating_sub(1);
        self.残りフレーム == 0
    }
}

impl シーン描画資源 {
    /// 注意: 同じIDの束を解除せずに追加するのは呼び出し元のバグである。上書きを許すと旧い束のGPU資源を辿れなくなり漏れるため、回復可能な失敗ではなく破られた不変条件として扱う。
    pub(in crate::renderer) fn 束を追加する(
        &mut self,
        device: &GPUデバイス,
        材料: 束追加材料<'_>,
        id: 描画束ID,
        描画対象一覧: &[描画対象素材],
    ) -> Result<(), レンダラーエラー> {
        assert!(
            !self.チャンク一覧.iter().any(|束| 束.id() == id),
            "同じ束ID{}を解除せずに二重追加した",
            id.番号を返す()
        );
        let 束 = チャンク描画資源::生成する(device, 材料, id, 描画対象一覧)?;
        self.チャンク一覧.push(束);
        Ok(())
    }

    /// 束を描画対象から外して破棄待ちへ移す。実際の破棄は`破棄待ちを一フレーム進める`が行う。
    /// 該当するIDが無ければ`false`を返す。呼び出し元が解除済みの束を再び解除しても異常ではない。
    pub(in crate::renderer) fn 束を解除予約する(&mut self, id: 描画束ID) -> bool {
        let Some(位置) = self.チャンク一覧.iter().position(|束| 束.id() == id) else {
            return false;
        };
        let 束 = self.チャンク一覧.remove(位置);
        self.破棄待ち.push(破棄待ち束 {
            束,
            残りフレーム: 進行中フレーム数,
        });
        true
    }

    pub(in crate::renderer) fn 破棄待ち件数(&self) -> usize {
        self.破棄待ち.len()
    }

    /// 前提: 呼び出し元がGPUの全作業完了を待ってから呼ぶ。提示が止まって`破棄待ちを一フレーム進める`が呼ばれない間に溜まった束を、フレームの進行に頼らず解放するための経路である。
    pub(in crate::renderer) fn 破棄待ちを全て破棄する(&mut self, device: &GPUデバイス) {
        for 待ち in self.破棄待ち.drain(..) {
            待ち.束.破棄する(device);
            self.実破棄済みid一覧.push(待ち.束.id());
        }
    }

    /// 前提: 1フレームにつきちょうど1回、描画発行の前に呼ぶ。呼ばないと破棄待ちが解放されず、多く呼ぶとGPU使用中の破棄になる。
    pub(in crate::renderer) fn 破棄待ちを一フレーム進める(&mut self, device: &GPUデバイス) {
        let mut 残す = Vec::with_capacity(self.破棄待ち.len());
        for mut 待ち in self.破棄待ち.drain(..) {
            if 待ち.一フレーム進める() {
                待ち.束.破棄する(device);
                self.実破棄済みid一覧.push(待ち.束.id());
            } else {
                残す.push(待ち);
            }
        }
        self.破棄待ち = 残す;
    }

    /// 実際に破棄し終えた束のIDを受け皿へ移して空にする。資源の会計を実破棄の時点で減らす唯一の観測点である。
    pub(in crate::renderer) fn 実破棄済みを引き取る(&mut self, 受け皿: &mut Vec<描画束ID>) {
        受け皿.append(&mut self.実破棄済みid一覧);
    }
}
