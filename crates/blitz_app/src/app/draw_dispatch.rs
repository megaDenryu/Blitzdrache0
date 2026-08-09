//! スモークアクションに応じた描画呼び出しの振り分け(通常描画/読み戻し判定)と、そのフレームの材料から描画入力を組み立てて実行する入口。
//! 入口を1つ置くのは、地形詳細段選択と可視個体選択の受け皿がアプリの外へ預けられている間だけ描画入力を作れるためであり、
//! 呼び出し元は預けと戻しだけを見ればよい。

mod execute;
mod frame_reach;
mod local_visibility_check;
mod motion_vector_check;
mod pixel_readback;
mod rewrite_action;
mod temporal_reconstruction_check;
pub(crate) mod temporal_reconstruction_injection;

use blitz_render::frame_input::プリミティブ発行受け皿;
pub(super) use frame_reach::描画の到達;
pub(super) use temporal_reconstruction_check::時間再構成の観測;
pub(crate) use temporal_reconstruction_injection::canon::突き合わせの要約 as 時間再構成の突き合わせの要約;

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
    pub(super) 動く個体の大域位置の指定一覧: &'a [blitz_render::frame_input::動く個体の大域位置の指定],
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
            材料.動く個体の大域位置の指定一覧,
        );
        描画入力.布 = 材料.布入力;
        描画入力.ui描画 = 材料.ui描画;
        let 到達 = if let Some(形) = self.局所可視性を検収する形(材料.アクション) {
            self.局所可視度を検収する(描画入力, 材料.視点情報, 形)?
        } else if self.動きベクトルを報告するフレームか(材料.アクション) {
            self.動きベクトルを報告する(描画入力)?
        } else if self.時間再構成の合成入力を突き合わせるフレームか(材料.アクション) {
            self.時間再構成の合成入力を突き合わせる(描画入力, 材料.視点情報)?
        } else if self.時間再構成を観測するフレームか(材料.アクション) {
            self.時間再構成を観測する(描画入力)?
        } else if self.ダンプ対象フレームか(材料.アクション) {
            self.読み戻してダンプする(描画入力, 材料.視点情報, 材料.アクション)?
        } else {
            self.実行して判定する(材料.アクション, 描画入力)?
        };
        // 生成パスがGPUへ届いたフレームだけが鍵を確定できる。届かなかったフレームで確定すると、
        // 焼いていない鍵を焼いたことにして次の実描画が「前の結果を使う」と答える。
        // 視点の履歴も同じ規律で確定する。履歴画像を最後に書いたのは提示まで到達したフレームであり、
        // 見送られたフレームの視点を記録すると、次のフレームの動きベクトルと棄却が誤った前フレーム対応を使う
        // (`参照: _doc/設計/時間再構成.md`「判断c: 履歴画像の持ち方と無効化の規律」)。
        if 到達 == 描画の到達::提示した {
            self.天空.焼き上げを確定する();
            self.視点の履歴
                .記録する(材料.視点情報.ビュー射影の組.ずらし無し(), 材料.視点情報.カメラ大域位置);
        }
        Ok(())
    }
}
