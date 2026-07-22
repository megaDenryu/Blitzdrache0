//! 開発用UI(egui)の1フレームぶんの描画データ組み立て。`frame`の行数分割のための切り出し。

use super::アプリ;
use crate::dev_ui::stats::開発UI統計;
use crate::error::起動エラー;

impl アプリ {
    /// ウィンドウ・レンダラー・開発UIのいずれかが未生成なら`None`(起動直後の1フレーム目のみ起こりうる)。
    pub(super) fn ui描画データを組み立てる(&mut self) -> Result<Option<blitz_render::UI描画データ>, 起動エラー> {
        let Some(window) = &self.window else { return Ok(None) };
        let Some(開発ui) = &mut self.開発ui else { return Ok(None) };
        let Some(レンダラー) = &mut self.レンダラー else {
            return Ok(None);
        };
        let 統計 = 開発UI統計 {
            パス別gpu時間: レンダラー.パス別gpu時間を取得する(),
            フレーム時間ms: 開発ui.フレーム時間を記録する(),
            validation件数: レンダラー.検証カウンタを取得する().件数を読む(),
        };
        開発ui.描画データを作る(window, レンダラー, 統計, &mut self.露出, &mut self.ブレンド)
    }
}
