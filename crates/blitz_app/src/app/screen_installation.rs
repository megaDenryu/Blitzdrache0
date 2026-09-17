//! 画面の据え付け。レンダラー・画面へ重ねるUI・ウィンドウの3つは、起動の途中(カタログを読んだ後)に
//! `resume`の同じ地点で揃って据わり、終了時に同じ地点で揃って消える。同時に生まれて同時に消える状態を
//! 別々の`Option`で持つと「一部だけ据わっている」という起こり得ない組み合わせが型の上で表現可能になるため、
//! 1つの型へまとめる。
//! 参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「判断11」、グローバルCLAUDE.md「不在・未設定・使用不可」。

use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::window::Window;

use blitz_render::{UI描画データ, ウィンドウ寸法, レンダラー};

use crate::error::起動エラー;
use crate::overlay_ui::stats::開発UI統計;
use crate::overlay_ui::{画面へ重ねるUI, 画面へ重ねる内容};

/// 注意: フィールドはこの宣言順を保つ。Rustは構造体フィールドを宣言順にDropするため、この順序が
/// レンダラー破棄(surface等)をウィンドウ破棄より必ず先に行うことを保証する(レンダラーの生成前提を満たす)。
pub(in crate::app) struct 画面の据え付け {
    レンダラー: レンダラー,
    画面へ重ねるui: 画面へ重ねるUI,
    window: Window,
}

impl 画面の据え付け {
    pub(in crate::app) fn 据える(レンダラー: レンダラー, 画面へ重ねるui: 画面へ重ねるUI, window: Window) -> Self {
        Self { レンダラー, 画面へ重ねるui, window }
    }

    /// 描画の下流が据え付けを無条件に借りる口。
    ///
    /// 注意: 据え付けの無い描画機会は描画の入口(`一フレーム実行する`)が弾く。その不変条件が破れたときは
    /// 回復のしようが無いため、`Option`を返して呼び出し側へ到達しない防御を書かせるのではなく、ここで落とす。
    /// 枠(`Option`)を引数で受けるのは、呼び出し側が`アプリ`の他のフィールドと同時に借りられるようにするためである。
    pub(in crate::app) fn 描画の最中に借りる(枠: &mut Option<Self>) -> &mut Self {
        match 枠 {
            Some(据え付け) => 据え付け,
            None => panic!("描画の下流が据え付けの無い状態へ到達した(描画の入口が据え付けの無い描画機会を弾く不変条件が破れている)"),
        }
    }

    /// `描画の最中に借りる`の読み取りだけの対。不変条件も落ちる理由も同じである。
    pub(in crate::app) fn 描画の最中に見る(枠: &Option<Self>) -> &Self {
        match 枠 {
            Some(据え付け) => 据え付け,
            None => panic!("描画の下流が据え付けの無い状態へ到達した(描画の入口が据え付けの無い描画機会を弾く不変条件が破れている)"),
        }
    }

    pub(in crate::app) fn レンダラーを借りる(&mut self) -> &mut レンダラー {
        &mut self.レンダラー
    }

    pub(in crate::app) fn レンダラーを見る(&self) -> &レンダラー {
        &self.レンダラー
    }

    pub(in crate::app) fn ウィンドウを見る(&self) -> &Window {
        &self.window
    }

    pub(in crate::app) fn ウィンドウの物理寸法(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub(in crate::app) fn 次の描画を要求する(&self) {
        self.window.request_redraw();
    }

    pub(in crate::app) fn 画面の寸法の変化をレンダラーへ通知する(&mut self, 寸法: ウィンドウ寸法) {
        self.レンダラー.サイズ変更を通知する(寸法);
    }

    /// eguiがこのwinitイベントを消費したかを返す。消費されたイベントは呼び出し元がカメラ入力へ流さない。
    pub(in crate::app) fn uiがwinitイベントを消費したか(&mut self, event: &WindowEvent) -> bool {
        self.画面へ重ねるui.winitイベントを取り込む(&self.window, event)
    }

    pub(in crate::app) fn 開発パネルの表示を切り替える(&mut self) {
        self.画面へ重ねるui.開発パネルの表示を切り替える();
    }

    /// 開発パネルへ載せる統計。レンダラーの計器とUI自身のフレーム時間の両方を読むため、この型が組む。
    pub(in crate::app) fn 開発パネルの統計を採る(&mut self) -> 開発UI統計 {
        開発UI統計 {
            パス別gpu時間: self.レンダラー.パス別gpu時間を取得する(),
            フレーム時間ms: self.画面へ重ねるui.フレーム時間を記録する(),
            検証件数: self.レンダラー.検証カウンタを取得する().件数を読む(),
        }
    }

    /// このフレームぶんのUI描画データを組む。`露出`と`ブレンド`はスライダーが書き換えるため可変で受ける。
    pub(in crate::app) fn ui描画データを作る(
        &mut self, 内容: 画面へ重ねる内容, 露出: &mut crate::cli::露出倍率, ブレンド: &mut crate::cli::アニメーションのブレンド係数
    ) -> Result<Option<UI描画データ>, 起動エラー> {
        self.画面へ重ねるui.描画データを作る(&self.window, &mut self.レンダラー, 内容, 露出, ブレンド)
    }
}
