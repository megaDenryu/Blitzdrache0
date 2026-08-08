//! 合成入力を注入してCPU正本とGPUの結果を突き合わせる工程。受け取るのはそのフレームの描画入力と視点、
//! 返すのはそのフレームが提示へ到達したかである。
//!
//! 同じ入力で2フレーム描くのは、1枚目で履歴を有効にするためである。確保直後と無効化直後の履歴は混ぜない規律であり、
//! 1枚目だけでは混合が1画素も働かない。2枚目が読む履歴は注入が書き戻した4枚のうちの1枚であるため、
//! 1枚目が何を書いたかは2枚目の結果に現れない。
//!
//! 深度も同じ1本のパスで書き戻すのは、局所可視性補正の合成深度の注入が深度プリパスの直後へ積まれ、その後のシーン描画が
//! 深度プリパスの方式によっては同じ画像を書き直すためである。時間再構成が読む地点で値が決まっていることを、
//! 4枚を同時に書き戻すことで保証する。
//! 参照: `_doc/設計/時間再構成.md`「検収戦略(判断g)」の2

pub(crate) mod canon;
mod synthesis;

use blitz_render::temporal_reconstruction::時間再構成の合成入力;
use blitz_render::{フレーム描画入力, 読み戻し結果};

use super::frame_reach::描画の到達;
use crate::app::frame::フレーム視点;
use crate::app::アプリ;
use crate::cli::起動モード;
use crate::error::起動エラー;
use crate::reports::temporal_reconstruction;
use crate::smoke::スモークアクション;

impl アプリ {
    /// このフレームで合成入力の突き合わせを行うか。`--report-temporal-reconstruction-injection`を与えた
    /// `--frames`の最終フレームだけが対象である。
    pub(in crate::app) fn 時間再構成の合成入力を突き合わせるフレームか(
        &self, アクション: スモークアクション
    ) -> bool {
        if !self.読み戻し検収.時間再構成の合成入力を突き合わせるか {
            return false;
        }
        let 起動モード::スモーク実行 { フレーム数 } = self.起動モード else {
            return false;
        };
        self.現在フレーム + 1 == フレーム数 && アクション != スモークアクション::差し替え前ダンプ
    }

    pub(in crate::app) fn 時間再構成の合成入力を突き合わせる(
        &mut self,
        描画入力: フレーム描画入力<'_>,
        視点情報: &フレーム視点,
    ) -> Result<描画の到達, 起動エラー> {
        let 射影 = 視点情報.射影の復元;
        let Some(レンダラー) = &mut self.レンダラー else {
            return Ok(描画の到達::届かなかった);
        };
        let 寸法 = レンダラー.画面の寸法().map_err(描画側の失敗へ写す)?;
        let 材料 = synthesis::焼く(寸法, 射影);
        let 合成入力 = 時間再構成の合成入力::生成する(
            寸法,
            材料.今のフレームの色.clone(),
            材料.履歴.clone(),
            材料.動きベクトル.clone(),
            材料.深度.clone(),
        )
        .map_err(|誤り| 起動エラー::from(blitz_render::レンダラーエラー::from(誤り)))?;
        レンダラー.時間再構成の合成入力を注入する(&合成入力)?;
        // 1枚目は履歴を有効にするためだけに描く。読み戻すのは、見送られたフレームを行に残す経路を2枚で揃えるためである。
        if 読み戻す(レンダラー, 描画入力.clone())?.is_none() {
            return Ok(描画の到達::届かなかった);
        }
        let Some(結果) = 読み戻す(レンダラー, 描画入力)? else {
            return Ok(描画の到達::届かなかった);
        };
        let 要約 = canon::突き合わせる(&材料, 寸法, 射影, &結果);
        temporal_reconstruction::注入の突き合わせを報告する(&要約);
        Ok(描画の到達::提示した)
    }
}

fn 読み戻す(
    レンダラー: &mut blitz_render::レンダラー,
    描画入力: フレーム描画入力<'_>,
) -> Result<Option<blitz_render::HDR読み戻し画像>, 起動エラー> {
    match レンダラー.一フレーム描画して圧縮前のhdrを読み戻す(描画入力)? {
        読み戻し結果::読み戻した(画像) => Ok(Some(画像)),
        読み戻し結果::見送った(理由) => {
            temporal_reconstruction::見送りを報告する("合成入力の突き合わせ", &format!("{理由:?}"));
            Ok(None)
        }
    }
}

/// 合成入力の値域違反を描画側の失敗として運ぶ。寸法も値も描画側が決めた形であり、アプリ層に固有の失敗ではない。
fn 描画側の失敗へ写す(誤り: blitz_render::local_visibility::局所可視性エラー) -> 起動エラー {
    blitz_render::レンダラーエラー::from(誤り).into()
}
