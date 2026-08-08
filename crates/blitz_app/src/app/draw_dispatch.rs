//! スモークアクションに応じた描画呼び出しの振り分け(通常描画/読み戻し判定)と、そのフレームの材料から描画入力を組み立てて実行する入口。
//! 入口を1つ置くのは、地形詳細段選択と可視個体選択の受け皿がアプリの外へ預けられている間だけ描画入力を作れるためであり、
//! 呼び出し元は預けと戻しだけを見ればよい。

mod execute;
mod frame_reach;
mod local_visibility_check;
mod motion_vector_check;
mod pixel_readback;
mod rewrite_action;

use blitz_render::frame_input::プリミティブ発行受け皿;
pub(super) use frame_reach::描画の到達;

use blitz_render::{UI描画データ, 布フレーム入力};

use super::visibility::可視選択受け皿;
use super::アプリ;
use crate::error::起動エラー;
use crate::smoke::スモークアクション;

/// 1フレームの描画に要る、アプリの外へ預けてある値と組み立て済みの値の束。
pub(super) struct フレーム材料<'a> {
    pub(super) アクション: スモークアクション,
    pub(super) 視点情報: &'a super::frame::フレーム視点,
    pub(super) 地形詳細段選択一覧: &'a [blitz_render::地形詳細段選択],
    pub(super) 可視受け皿: &'a 可視選択受け皿,
    pub(super) プリミティブ発行: &'a プリミティブ発行受け皿,
    pub(super) 布入力: Option<布フレーム入力>,
    pub(super) ui描画: Option<UI描画データ>,
}

impl アプリ {
    pub(super) fn 描画まで進める(&mut self, 材料: フレーム材料<'_>) -> Result<(), 起動エラー> {
        let 可視個体選択一覧 =
            blitz_render::可視個体選択一覧::生成する(材料.可視受け皿.選択一覧(), 材料.可視受け皿.id列(), 材料.可視受け皿.段範囲列())?;
        // 焼き直しの判定は保留中の鍵を書き換えるため、借用が重ならないよう描画入力の組み立てより前に済ませる。
        let 焼き上げ入力 = self.天空.焼き上げ入力(材料.視点情報);
        let mut 描画入力 = super::frame::描画入力を組み立てる(
            self,
            焼き上げ入力,
            材料.視点情報,
            材料.地形詳細段選択一覧,
            可視個体選択一覧,
            材料.プリミティブ発行,
        );
        描画入力.布 = 材料.布入力;
        描画入力.ui描画 = 材料.ui描画;
        let 到達 = if let Some(形) = self.局所可視性を検収する形(材料.アクション) {
            self.局所可視度を検収する(描画入力, 材料.視点情報, 形)?
        } else if self.動きベクトルを報告するフレームか(材料.アクション) {
            self.動きベクトルを報告する(描画入力)?
        } else if self.ダンプ対象フレームか(材料.アクション) {
            self.読み戻してダンプする(描画入力, 材料.視点情報, 材料.アクション)?
        } else {
            self.実行して判定する(材料.アクション, 描画入力)?
        };
        // 生成パスがGPUへ届いたフレームだけが鍵を確定できる。届かなかったフレームで確定すると、
        // 焼いていない鍵を焼いたことにして次の実描画が「前の結果を使う」と答える。
        if 到達 == 描画の到達::提示した {
            self.天空.焼き上げを確定する();
        }
        Ok(())
    }
}
