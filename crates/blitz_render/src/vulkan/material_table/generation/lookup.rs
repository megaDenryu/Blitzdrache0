//! 資源表世代の中で材質IDからレコードを引く操作。触れるのは世代番号・材質レコード列・材質別レコード添字の3つだけであり、
//! 画像集合にも束縛先にも触れない。
//!
//! 不変条件: 生の添字が外へ出るのは、束縛する世代と参照の世代が同じであることを確かめた後だけである。
//! 確かめずに渡すと、世代を作り直した後の表で旧い添字が別の材質を指す
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「材質レコードとテクスチャ台帳」)。

use crate::error::材質資源表エラー;

use super::資源表世代;
use crate::vulkan::material_table::generation_record::世代内材質レコード;
use crate::vulkan::material_table::material_gpu_reference::材質GPU参照;
use crate::vulkan::material_table::material_id::大域材質ID;
use crate::vulkan::material_table::record_index::材質レコード添字;

impl<画像, 付属> 資源表世代<画像, 付属> {
    /// 永続する束が持つ材質IDを、この世代の中での位置へ解決する。
    pub(crate) fn 解決する(&self, 材質id: 大域材質ID) -> Result<材質GPU参照, 材質資源表エラー> {
        let Some(添字) = self.材質別レコード添字.get(&材質id) else {
            return Err(材質資源表エラー::未知の材質ID { 材質id: 材質id.値() });
        };
        Ok(材質GPU参照::生成する(self.世代id(), *添字))
    }

    /// 描画発行の描画定数を作るときに、束縛するこの世代と参照の世代が同じであることを確かめてから添字を渡す。
    pub(crate) fn 描画へ渡すレコード添字(&self, 参照: 材質GPU参照) -> Result<材質レコード添字, 材質資源表エラー> {
        if 参照.世代id() != self.世代id() {
            return Err(材質資源表エラー::異世代の混在 {
                参照の世代: 参照.世代id().番号(),
                束縛する世代: self.世代id().番号(),
            });
        }
        Ok(参照.レコード添字())
    }

    pub(crate) fn レコード(&self, 添字: 材質レコード添字) -> Option<&世代内材質レコード> {
        self.材質レコード列.get(添字.配列添字())
    }

    /// 世代の状態遷移の検査だけが数える。本番の経路は材質IDから1件を解決するため、件数を知る必要がない。
    #[cfg(test)]
    pub(crate) fn 材質件数(&self) -> usize {
        self.材質レコード列.len()
    }
}
