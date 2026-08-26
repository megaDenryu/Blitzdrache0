//! 現在読み込まれている全ての描画束の材質を、資源表世代を何度でも作り直せる形で保持する台帳。
//! 担当するのは、材質・テクスチャ・画像へ安定IDを発番することと、束の追加と解除で保持する材質を出し入れすることである。
//!
//! CPU側の画素を保持するのは、資源表世代がレイアウトを固定したまま丸ごと作り直される設計だからである。
//! 束が1つ増えるたびに全材質を梱包し直すため、素材を捨ててしまうと作り直せない。テクスチャを持たない材質は
//! 画素を1バイトも保持しない(「無し」は正準フォールバックが引き受けるため素材を要さない)。
//! `束別`の並びが梱包の順であり、そのまま世代内のレコード添字の並びになる。
//! 束ごとの並びの保持は`entry`、材質1件の形と出入りは`held_material`、IDの発番と画像の重複除去は`minting`にある。
//! 参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「材質レコードとテクスチャ台帳」

mod entry;
mod held_material;
mod minting;

use crate::draw_bundle_id::描画束ID;
use crate::render_object_material::描画対象素材;

use super::pack_input::梱包対象材質;

use minting::安定IDの発番;

pub(in crate::vulkan::material_table) use entry::描画対象別の材質ID;
pub(crate) use entry::束の保持材質;

pub(in crate::vulkan::material_table) struct 材質登録簿 {
    束別: Vec<(描画束ID, 束の保持材質)>, // 束ごとの保持材質
    発番: 安定IDの発番,
}

impl 材質登録簿 {
    pub(in crate::vulkan::material_table) fn 新規() -> Self {
        Self {
            束別: Vec::new(),
            発番: 安定IDの発番::新規(),
        }
    }

    /// 束1つぶんの材質を保持し、描画対象ごと材質スロットごとに発番した大域材質IDを返す。
    /// 注意: 同じ束IDを解除せずに登録するのは呼び出し元のバグである。通すと解除で片方だけが消える。
    pub(in crate::vulkan::material_table) fn 束を登録する(
        &mut self,
        束id: 描画束ID,
        描画対象一覧: &[描画対象素材],
    ) -> Vec<描画対象別の材質ID> {
        assert!(
            !self.束別.iter().any(|(既存, _)| *既存 == 束id),
            "同じ描画束{}の材質を解除せずに二重登録した",
            束id.番号を返す()
        );
        let 保持 = 束の保持材質::取り込む(&mut self.発番, 描画対象一覧);
        let 材質id一覧 = 保持.描画対象別の材質id一覧();
        self.束別.push((束id, 保持));
        材質id一覧
    }

    /// 該当する束の保持材質を捨てる。無ければ`false`を返す。解除済みの束を再び解除しても異常ではない。
    pub(in crate::vulkan::material_table) fn 束を解除する(&mut self, 束id: 描画束ID) -> bool {
        self.束を取り外す(束id).is_some()
    }

    /// 該当する束の保持材質を、並びの中の位置と一緒に所有権ごと取り出す。位置を返すのは、差し替えが失敗したときに
    /// 取り外す前とまったく同じ並びへ戻すためである。並びは梱包の順であり、順が変わると材質レコード添字も変わる。
    pub(in crate::vulkan::material_table) fn 束を取り外す(&mut self, 束id: 描画束ID) -> Option<(usize, 束の保持材質)> {
        let 位置 = self.束別.iter().position(|(既存, _)| *既存 == 束id)?;
        Some((位置, self.束別.remove(位置).1))
    }

    /// 取り外した保持材質を元の位置へ戻す。注意: 同じ束IDが既にあるところへ戻すのは呼び出し元のバグである。
    pub(in crate::vulkan::material_table) fn 束を位置へ戻す(&mut self, 位置: usize, 束id: 描画束ID, 保持: 束の保持材質) {
        assert!(
            !self.束別.iter().any(|(既存, _)| *既存 == 束id),
            "取り外していない描画束{}へ保持材質を戻そうとした",
            束id.番号を返す()
        );
        assert!(位置 <= self.束別.len(), "取り外したときの位置が現在の並びの外だった");
        self.束別.insert(位置, (束id, 保持));
    }

    /// 束IDの並び。取り外しと戻しが並びを保つことの検査だけが読む。
    #[cfg(test)]
    pub(in crate::vulkan::material_table) fn 束idの並び(&self) -> Vec<描画束ID> {
        self.束別.iter().map(|(束id, _)| *束id).collect()
    }

    pub(in crate::vulkan::material_table) fn 全て解除する(&mut self) {
        self.束別.clear();
    }

    /// 保持中の全材質を、世代の構築が受け取る梱包対象材質の並びとして借りる。
    pub(in crate::vulkan::material_table) fn 梱包対象を並べる(&self) -> Vec<梱包対象材質<'_>> {
        self.束別.iter().flat_map(|(_, 保持)| 保持.梱包対象を並べる()).collect()
    }
}
