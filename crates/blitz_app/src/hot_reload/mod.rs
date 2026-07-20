//! ホットリロード: シェーダーソースと、現在シーンの参照ファイル一覧を
//! std のみのmtimeポーリングで監視する。新規依存クレートは追加しない。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断7」「判断22」。

mod asset_watch;
mod compile;
mod mtime;
mod shader_watch;
mod slangc;

use std::path::PathBuf;
use std::time::{Duration, Instant};

use blitz_engine::{アセットID, カタログ, シーンデータ};
use blitz_render::シェーダー一式;

use asset_watch::アセット監視状態;
use shader_watch::{シェーダー変化結果, シェーダー監視状態};

const 確認間隔: Duration = Duration::from_millis(500);

/// `ホットリローダー::確認する` の結果。
pub(crate) enum ホットリロード結果 {
    変化なし,
    シェーダー再コンパイル成功 { シェーダー: シェーダー一式 },
    シェーダー再コンパイル失敗 { メッセージ: String },
    アセット再読込成功 { シーン: シーンデータ },
    アセット再読込失敗 { メッセージ: String },
}

/// シェーダーソース・シーン参照ファイルの両方を約0.5秒間隔でポーリングする。
/// どちらも監視対象が無い(パス不在・アセット未設定)場合は監視を無効化する。
pub(crate) struct ホットリローダー {
    シェーダー監視: Option<シェーダー監視状態>,
    アセット監視: Option<アセット監視状態>,
    前回確認時刻: Instant,
}

impl ホットリローダー {
    pub(crate) fn 生成する(シェーダー監視パス: PathBuf) -> Self {
        Self {
            シェーダー監視: shader_watch::構築する(シェーダー監視パス),
            アセット監視: None,
            前回確認時刻: Instant::now(),
        }
    }

    /// 初回シーン読込成功後に呼び、アセットの監視対象を設定する
    /// (カタログ・シーン読込はウィンドウ・レンダラー生成後にしか確定しないため)。
    pub(crate) fn アセット監視を設定する(&mut self, カタログ: カタログ, id: アセットID, 参照ファイル一覧: &[PathBuf]) {
        self.アセット監視 = Some(asset_watch::構築する(カタログ, id, 参照ファイル一覧));
    }

    pub(crate) fn 確認する(&mut self) -> ホットリロード結果 {
        let 今 = Instant::now();
        if 今.duration_since(self.前回確認時刻) < 確認間隔 {
            return ホットリロード結果::変化なし;
        }
        self.前回確認時刻 = 今;

        if let Some(監視) = &mut self.シェーダー監視 {
            match 監視.変化を確認する() {
                シェーダー変化結果::変化なし => {}
                シェーダー変化結果::成功 { シェーダー } => {
                    return ホットリロード結果::シェーダー再コンパイル成功 { シェーダー };
                }
                シェーダー変化結果::失敗 { メッセージ } => {
                    return ホットリロード結果::シェーダー再コンパイル失敗 { メッセージ };
                }
            }
        }

        if let Some(監視) = &mut self.アセット監視
            && let Some(結果) = 監視.変化を確認して再読込する()
        {
            return match 結果 {
                Ok(シーン) => ホットリロード結果::アセット再読込成功 { シーン },
                Err(誤り) => ホットリロード結果::アセット再読込失敗 { メッセージ: 誤り.to_string() },
            };
        }

        ホットリロード結果::変化なし
    }
}
