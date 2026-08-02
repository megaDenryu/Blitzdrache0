//! 完全に構築済みの資源表の1つの版。担当するのは、材質レコード列と画像集合を1つの不変な所有物として保ち、
//! 材質IDの解決と束縛する世代の一致検査を行うことである。
//!
//! 不変条件: 生成後に内容を変えない。フレームが束縛するのは構築済みの世代だけであるという規律が、
//! GPU使用中のディスクリプタ書き換え(update-after-bind)を初期要件から外す根拠である。
//! 参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「ディスクリプタ索引の採用範囲」

use std::collections::HashMap;

use crate::error::材質資源表エラー;

use super::generation_id::資源表世代ID;
use super::generation_record::世代内材質レコード;
use super::material_gpu_reference::材質GPU参照;
use super::material_id::大域材質ID;
use super::record_index::材質レコード添字;

pub(crate) struct 資源表世代<画像> {
    世代id: 資源表世代ID,
    材質レコード列: Vec<世代内材質レコード>,
    材質別レコード添字: HashMap<大域材質ID, 材質レコード添字>,
    /// テクスチャスロットの添字で引く画像。台帳の発番順と同じ並びである。
    画像集合: Vec<画像>,
}

impl<画像> 資源表世代<画像> {
    pub(in crate::vulkan::material_table) fn 束ねる(
        世代id: 資源表世代ID,
        材質レコード列: Vec<世代内材質レコード>,
        材質別レコード添字: HashMap<大域材質ID, 材質レコード添字>,
        画像集合: Vec<画像>,
    ) -> Self {
        Self {
            世代id,
            材質レコード列,
            材質別レコード添字,
            画像集合,
        }
    }

    /// 破棄の経路だけが使う、材質も画像も持たない世代。公開中の世代を取り出すときの置き換え先である。
    pub(in crate::vulkan::material_table) fn 空にする(世代id: 資源表世代ID) -> Self {
        Self::束ねる(世代id, Vec::new(), HashMap::new(), Vec::new())
    }

    pub(crate) const fn 世代id(&self) -> 資源表世代ID {
        self.世代id
    }

    /// 永続する束が持つ材質IDを、この世代の中での位置へ解決する。
    pub(crate) fn 解決する(&self, 材質id: 大域材質ID) -> Result<材質GPU参照, 材質資源表エラー> {
        let Some(添字) = self.材質別レコード添字.get(&材質id) else {
            return Err(材質資源表エラー::未知の材質ID { 材質id: 材質id.値() });
        };
        Ok(材質GPU参照::生成する(self.世代id, *添字))
    }

    /// フレームのpacketを組み立てるときに、束縛するこの世代と参照の世代が同じであることを確かめてから添字を渡す。
    pub(crate) fn 描画へ渡すレコード添字(&self, 参照: 材質GPU参照) -> Result<材質レコード添字, 材質資源表エラー> {
        if 参照.世代id() != self.世代id {
            return Err(材質資源表エラー::異世代の混在 {
                参照の世代: 参照.世代id().番号(),
                束縛する世代: self.世代id.番号(),
            });
        }
        Ok(参照.レコード添字())
    }

    pub(crate) fn レコード(&self, 添字: 材質レコード添字) -> Option<&世代内材質レコード> {
        self.材質レコード列.get(添字.配列添字())
    }

    pub(crate) fn 材質件数(&self) -> usize {
        self.材質レコード列.len()
    }

    pub(crate) fn 画像枚数(&self) -> usize {
        self.画像集合.len()
    }

    /// 退役の経路だけが使う、画像の所有権の受け渡し。取り出した後の世代は解決に使わない。
    pub(in crate::vulkan::material_table) fn 画像集合を取り出す(self) -> Vec<画像> {
        self.画像集合
    }
}
