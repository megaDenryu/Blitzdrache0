//! テクスチャIDから資源表世代内のスロットへの解決。担当するのは、スロットの発番と、同じ画像を指す複数のIDの重複除去である。
//! 画像そのものは持たず、発番した順番と世代の画像集合の要素番号が1対1で対応することだけを保つ。
//!
//! 不変条件: 発番したスロットは0から連番であり、その順に世代の画像集合へ画像が積まれる。
//! 注意: 同じテクスチャIDが別の画像またはビュー契約で現れたら型付きの失敗にする。無言で先勝ちにすると、
//! 同じIDが指す画像が呼び出し順で変わる。容量を決めるための必要枚数の見積は`required_count`が持つ。

mod required_count;

use std::collections::HashMap;

use crate::error::材質資源表エラー;

use super::capacity::テクスチャ表容量;
use super::image_identity::画像同一性;
use super::texture_id::テクスチャID;
use super::texture_role::材質テクスチャ役割;
use super::texture_slot::テクスチャスロット;
use super::texture_spec::テクスチャ指定;

/// 引き当てたスロットへ、まだ画像を常駐させていないかどうかを添えて返す。呼び出し元はこの判別だけを見て画像を作る。
pub(in crate::vulkan::material_table) enum スロットの引き当て {
    既に常駐している(テクスチャスロット),
    常駐させる必要がある(テクスチャスロット),
}

pub(in crate::vulkan::material_table) use required_count::必要枚数を数える;

pub(crate) struct テクスチャ台帳 {
    id別: HashMap<テクスチャID, (画像同一性, テクスチャスロット)>,
    画像別: HashMap<画像同一性, テクスチャスロット>,
    発番済み: u32,
}

impl テクスチャ台帳 {
    pub(in crate::vulkan::material_table) fn 新規() -> Self {
        Self {
            id別: HashMap::new(),
            画像別: HashMap::new(),
            発番済み: 0,
        }
    }

    /// アセットのIDを持たない正準フォールバックのためのスロット。IDから解決できないため、用途からの解決は梱包工程だけが持つ。
    pub(in crate::vulkan::material_table) fn 台帳外のスロットを発番する(
        &mut self,
        容量: テクスチャ表容量,
    ) -> Result<テクスチャスロット, 材質資源表エラー> {
        self.発番する(容量)
    }

    pub(in crate::vulkan::material_table) fn 引き当てる(
        &mut self,
        指定: &テクスチャ指定<'_>,
        役割: 材質テクスチャ役割,
        容量: テクスチャ表容量,
    ) -> Result<スロットの引き当て, 材質資源表エラー> {
        let 同一性 = 画像同一性::生成する(指定.画像id(), 役割.ビュー契約());
        if let Some((既知の同一性, スロット)) = self.id別.get(&指定.テクスチャid()) {
            if *既知の同一性 != 同一性 {
                return Err(材質資源表エラー::テクスチャIDの衝突 {
                    テクスチャid: 指定.テクスチャid().値(),
                });
            }
            return Ok(スロットの引き当て::既に常駐している(*スロット));
        }
        if let Some(スロット) = self.画像別.get(&同一性).copied() {
            self.id別.insert(指定.テクスチャid(), (同一性, スロット));
            return Ok(スロットの引き当て::既に常駐している(スロット));
        }
        let スロット = self.発番する(容量)?;
        self.id別.insert(指定.テクスチャid(), (同一性, スロット));
        self.画像別.insert(同一性, スロット);
        Ok(スロットの引き当て::常駐させる必要がある(スロット))
    }

    /// テクスチャIDからスロットへの解決を行う唯一の口。
    pub(crate) fn 解決する(&self, テクスチャid: テクスチャID) -> Option<テクスチャスロット> {
        self.id別.get(&テクスチャid).map(|(_, スロット)| *スロット)
    }

    pub(crate) const fn 発番済みスロット数(&self) -> u32 {
        self.発番済み
    }

    fn 発番する(&mut self, 容量: テクスチャ表容量) -> Result<テクスチャスロット, 材質資源表エラー> {
        if self.発番済み >= 容量.枚数() {
            return Err(材質資源表エラー::容量超過 {
                必要枚数: self.発番済み.saturating_add(1),
                予算枚数: 容量.枚数(),
            });
        }
        let スロット = テクスチャスロット::生成する(self.発番済み);
        self.発番済み = self.発番済み.saturating_add(1);
        Ok(スロット)
    }
}
