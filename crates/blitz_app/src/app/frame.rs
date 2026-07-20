//! 1フレーム実行: ホットリロード確認 → スモークアクション判定 → 描画/判定。

use blitz_render::読み戻し結果;
use winit::event_loop::ActiveEventLoop;

use super::アプリ;
use crate::cli::起動モード;
use crate::error::起動エラー;
use crate::hot_reload::ホットリロード結果;
use crate::smoke::{self, スモークアクション};

impl アプリ {
    /// RedrawRequestedのたびに1フレーム分の描画を実行する。
    pub(super) fn 一フレーム実行する(&mut self, event_loop: &ActiveEventLoop) {
        if self.レンダラー.is_none() {
            return;
        }
        self.ホットリロードを確認する();

        let アクション = match self.起動モード {
            起動モード::スモーク実行 { フレーム数 } => smoke::判定する(self.現在フレーム, フレーム数),
            起動モード::無期限実行 => スモークアクション::通常描画,
        };
        if let Some(window) = &self.window {
            smoke::window自己操作を適用する(window, アクション);
        }

        if let Err(誤り) = self.実行して判定する(アクション) {
            self.起動時エラー = Some(誤り);
            event_loop.exit();
            return;
        }

        self.現在フレーム += 1;
        if let 起動モード::スモーク実行 { フレーム数 } = self.起動モード
            && self.現在フレーム >= フレーム数
        {
            event_loop.exit();
        }
    }

    fn ホットリロードを確認する(&mut self) {
        let 結果 = self.ホットリローダー.確認する();
        let Some(レンダラー) = &mut self.レンダラー else {
            return;
        };
        match 結果 {
            ホットリロード結果::変化なし => {}
            ホットリロード結果::再コンパイル成功 { シェーダー } => {
                if let Err(誤り) = レンダラー.シェーダーを差し替える(シェーダー) {
                    eprintln!("[hot-reload] パイプライン差し替えに失敗した: {誤り}");
                }
            }
            ホットリロード結果::再コンパイル失敗 { メッセージ } => {
                eprintln!("[hot-reload] シェーダーの再コンパイルに失敗した:\n{メッセージ}");
            }
        }
    }

    fn 実行して判定する(&mut self, アクション: スモークアクション) -> Result<(), 起動エラー> {
        if アクション == スモークアクション::シェーダー書き換え {
            smoke::シェーダーを書き換える(&self.シェーダー監視パス)?;
        }

        let Some(レンダラー) = &mut self.レンダラー else {
            return Ok(());
        };

        match アクション {
            スモークアクション::初期色判定 | スモークアクション::最終判定 => {
                match レンダラー.一フレーム描画して読み戻す(self.クリア色)? {
                    読み戻し結果::読み戻した(画像) => smoke::ピクセルを判定する(&画像, アクション),
                    読み戻し結果::見送った(理由) => Err(起動エラー::ピクセル判定失敗(format!(
                        "判定対象フレームで描画が見送られた: {理由:?}"
                    ))),
                }
            }
            _ => {
                レンダラー.一フレーム描画する(self.クリア色)?;
                Ok(())
            }
        }
    }
}
