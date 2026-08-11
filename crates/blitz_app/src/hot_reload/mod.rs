//! ホットリロード: シェーダーソースと、現在シーンの参照ファイル一覧を
//! std のみのmtimeポーリングで監視する。新規依存クレートは追加しない。
//! シェーダーはimportでモジュール分割されているため、監視はエントリファイル
//! 単体でなくエントリが属するディレクトリ内の全.slangファイルを対象にする
//! (dir_mtime参照)。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断7」「判断22」。

mod asset_watch;
mod compile;
mod compile_error;
mod dir_mtime;
mod mtime;
mod reload_result;
mod shader_watch;
mod slangc;
mod watched_shader;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use asset_watch::アセット監視状態;
use blitz_engine::{アセットID, カタログ};
pub(crate) use compile_error::シェーダー再コンパイルエラー;
pub(crate) use reload_result::ホットリロード結果;
use shader_watch::{シェーダー変化結果, シェーダー監視状態};
pub(crate) use watched_shader::監視するシェーダーの入口ファイル;

const 確認間隔: Duration = Duration::from_millis(500);

/// シェーダーソース・シーン参照ファイルの両方を約0.5秒間隔でポーリングする。
/// どちらも監視対象が無い(パス不在・アセット未設定)場合は監視を無効化する。
pub(crate) struct ホットリローダー {
    シェーダー監視: Option<シェーダー監視状態>,
    アセット監視: Option<アセット監視状態>,
    前回確認時刻: Instant,
}

impl ホットリローダー {
    pub(crate) fn 生成する(入口ファイル: &監視するシェーダーの入口ファイル) -> Self {
        Self {
            シェーダー監視: shader_watch::構築する(入口ファイル.パス().to_path_buf()),
            アセット監視: None,
            前回確認時刻: Instant::now(),
        }
    }

    /// 初回シーン読込成功後に呼び、アセットの監視対象を設定する
    /// (カタログ・シーン読込はウィンドウ・レンダラー生成後にしか確定しないため)。
    pub(crate) fn アセット監視を設定する(&mut self, カタログ: カタログ, id: アセットID, 参照ファイル一覧: &[PathBuf]) {
        self.アセット監視 = Some(asset_watch::構築する(カタログ, id, 参照ファイル一覧));
    }

    /// アセット監視が所有するカタログを読み取り専用で貸す。カタログは初回シーン読込で1つだけ作られる台帳であり、
    /// ストリーミングが自前にもう1つ読むと同じ関心の所有者が2つになるため、ここから借りる。
    /// 設定前(初回シーン読込前)は`None`を返す。
    pub(crate) fn カタログを参照する(&self) -> Option<&カタログ> {
        self.アセット監視.as_ref().map(asset_watch::アセット監視状態::カタログ)
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
                シェーダー変化結果::失敗 { 誤り } => {
                    return ホットリロード結果::シェーダー再コンパイル失敗 { 誤り };
                }
            }
        }

        if let Some(監視) = &mut self.アセット監視
            && let Some(結果) = 監視.変化を確認して再読込する()
        {
            return match 結果 {
                Ok(シーン) => ホットリロード結果::アセット再読込成功 {
                    シーン: Box::new(シーン)
                },
                Err(誤り) => ホットリロード結果::アセット再読込失敗 { 誤り },
            };
        }

        ホットリロード結果::変化なし
    }
}
