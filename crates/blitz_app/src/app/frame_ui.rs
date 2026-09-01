//! 画面へ重ねるUI(egui)の1フレームぶんの描画データ組み立て。
//! 触れるフィールドは`画面へ重ねるui`・`ゲーム配線`・`露出`・`ブレンド`・`window`・`レンダラー`に限る。露出とブレンドはスライダーが書き換えるため可変で渡す。
//! 呼び出しタイミング: `frame`の`一フレーム実行する`が描画入力の作成より前に呼ぶ(同じフレームの描画へスライダーの変更を反映するため)。

use super::アプリ;
use crate::error::起動エラー;
use crate::overlay_ui::stats::開発UI統計;
use crate::overlay_ui::画面へ重ねる内容;

impl アプリ {
    /// ウィンドウ・レンダラー・画面へ重ねるUIのいずれかが未生成なら`None`(起動直後の1フレーム目のみ起こりうる)。
    pub(super) fn ui描画データを組み立てる(&mut self) -> Result<Option<blitz_render::UI描画データ>, 起動エラー> {
        let Some(window) = &self.window else { return Ok(None) };
        let Some(画面へ重ねるui) = &mut self.画面へ重ねるui else {
            return Ok(None);
        };
        let Some(レンダラー) = &mut self.レンダラー else {
            return Ok(None);
        };
        let 内容 = 画面へ重ねる内容 {
            開発パネルの統計: 開発UI統計 {
                パス別gpu時間: レンダラー.パス別gpu時間を取得する(),
                フレーム時間ms: 画面へ重ねるui.フレーム時間を記録する(),
                validation件数: レンダラー.検証カウンタを取得する().件数を読む(),
            },
            ゲーム画面: self.ゲーム配線.画面へ重ねる表示内容を作る(),
            移動とカメラの計器: self.ゲーム配線.移動とカメラの計器を作る(),
        };
        画面へ重ねるui.描画データを作る(window, レンダラー, 内容, &mut self.露出, &mut self.ブレンド)
    }
}
