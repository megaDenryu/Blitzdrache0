//! 完全に構築済みの資源表の1つの版。担当するのは、材質レコード列と画像集合を1つの不変な所有物として保ち、
//! 材質IDの解決と束縛する世代の一致検査を行うことである。
//!
//! 不変条件: 生成後に内容を変えない。フレームが束縛するのは構築済みの世代だけであるという規律が、
//! GPU使用中のディスクリプタ書き換え(update-after-bind)を初期要件から外す根拠である。
//! 参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「ディスクリプタ索引の採用範囲」
//! `付属資源`は、画像とレコード列が確定した後に供給元が1度だけ仕上げる。
//! 材質IDからレコードを引く操作は`lookup`にある。

mod lookup;

use std::collections::HashMap;

use super::generation_id::資源表世代ID;
use super::generation_record::世代内材質レコード;
use super::generation_resolution::世代内材質解決;
use super::material_id::大域材質ID;

pub(crate) struct 資源表世代<画像, 付属> {
    世代id: 資源表世代ID,
    材質レコード列: Vec<世代内材質レコード>,
    材質別解決: HashMap<大域材質ID, 世代内材質解決>, // 材質IDから、レコード添字と正規化済みの材質変種キーへの対応
    画像集合: Vec<画像>,                             // テクスチャスロットの添字で引く画像。台帳の発番順と同じ並びである
    付属資源: Option<付属>,                          // この世代を束縛するための資源
}

impl<画像, 付属> 資源表世代<画像, 付属> {
    pub(in crate::vulkan::material_table) fn 束ねる(
        世代id: 資源表世代ID,
        材質レコード列: Vec<世代内材質レコード>,
        材質別解決: HashMap<大域材質ID, 世代内材質解決>,
        画像集合: Vec<画像>,
        付属資源: 付属,
    ) -> Self {
        Self {
            世代id,
            材質レコード列,
            材質別解決,
            画像集合,
            付属資源: Some(付属資源),
        }
    }

    /// 破棄の経路だけが使う、材質も画像も束縛先も持たない世代。公開中の世代を取り出すときの置き換え先である。
    pub(in crate::vulkan::material_table) fn 空にする(世代id: 資源表世代ID) -> Self {
        Self {
            世代id,
            材質レコード列: Vec::new(),
            材質別解決: HashMap::new(),
            画像集合: Vec::new(),
            付属資源: None,
        }
    }

    /// 束縛先。空の置き換え用の世代だけが持たないため、呼び出し元が公開中の世代に対して呼ぶ限り必ず在る。
    pub(crate) fn 付属資源(&self) -> &付属 {
        match &self.付属資源 {
            Some(資源) => 資源,
            None => panic!("束縛先を持たない置き換え用の資源表世代{}を束縛しようとした", self.世代id.番号()),
        }
    }

    pub(crate) const fn 世代id(&self) -> 資源表世代ID {
        self.世代id
    }

    pub(crate) fn 画像枚数(&self) -> usize {
        self.画像集合.len()
    }

    /// 退役の経路だけが使う、画像と束縛先の所有権の受け渡し。取り出した後の世代は解決に使わない。
    pub(in crate::vulkan::material_table) fn 資源を取り出す(self) -> (Vec<画像>, Option<付属>) {
        (self.画像集合, self.付属資源)
    }
}
