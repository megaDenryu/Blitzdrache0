//! 布フレーム入力(介入・カプセル定数)の毎フレーム書き込み(判断53・54)。
//! 布資源の有無と入力の有無の一致検証を含む。`draw_execute`の行数分割のための切り出し。

use super::レンダラー;
use crate::error::レンダラーエラー;
use crate::frame_input::フレーム描画入力;

impl レンダラー {
    /// 前提: 呼び出しはこのスロットのフェンス待ち後(判断24のUBO書き込みと同じ規律)。
    pub(super) fn 布フレームを書き込む(
        &self,
        フレーム添字: usize,
        入力: &フレーム描画入力,
    ) -> Result<(), レンダラーエラー> {
        match (&self.布, &入力.布) {
            (Some(布), Some(布入力)) => 布.フレーム入力を書き込む(&self.device, フレーム添字, 布入力),
            (None, None) => Ok(()),
            (Some(_), None) => Err(レンダラーエラー::布入力不一致("布付きレンダラーなのに布入力が未指定".to_string())),
            (None, Some(_)) => Err(レンダラーエラー::布入力不一致("布無しレンダラーに布入力が指定された".to_string())),
        }
    }

    /// 布の1フレーム描画入力を組み立てる(`draw_dispatch`用)。介入件数はフレーム入力から渡す。
    pub(super) fn 布入力を組み立てる(&self, フレーム添字: usize, 介入件数: u32) -> Option<crate::vulkan::frame::布描画入力> {
        self.布.as_ref().map(|一式| 一式.描画入力を作る(フレーム添字, 介入件数))
    }
}
