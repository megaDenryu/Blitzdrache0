//! シェーダーソースのmtime監視。変化検知でslangcを子プロセス実行し再コンパイルする。
//! 監視対象はエントリファイルが属するディレクトリ内の全.slangファイルの最大mtime
//! (importで分割されたpbr.slang等の変更も検知するため、単体ファイルのmtimeでは
//! 取りこぼす)。再コンパイル自体はエントリファイルをslangcに渡し、importの解決は
//! slangc自身にディレクトリ内で行わせる。

use std::path::PathBuf;
use std::time::SystemTime;

use blitz_render::シェーダー一式;

use super::{compile, dir_mtime};

/// シェーダー再コンパイルの結果。
pub(super) enum シェーダー変化結果 {
    変化なし,
    成功 { シェーダー: シェーダー一式 },
    失敗 { メッセージ: String },
}

pub(super) struct シェーダー監視状態 {
    エントリパス: PathBuf,
    監視ディレクトリ: PathBuf,
    最終更新時刻: SystemTime,
}

pub(super) fn 構築する(エントリパス: PathBuf) -> Option<シェーダー監視状態> {
    let 監視ディレクトリ = エントリパス.parent()?.to_path_buf();
    let 最終更新時刻 = dir_mtime::最新更新時刻を取得する(&監視ディレクトリ)?;
    Some(シェーダー監視状態 {
        エントリパス,
        監視ディレクトリ,
        最終更新時刻,
    })
}

impl シェーダー監視状態 {
    pub(super) fn 変化を確認する(&mut self) -> シェーダー変化結果 {
        let Some(現在更新時刻) = dir_mtime::最新更新時刻を取得する(&self.監視ディレクトリ) else {
            // 保存の途中でディレクトリが一時的に読めない等は無視し、次回の確認に委ねる。
            return シェーダー変化結果::変化なし;
        };
        if 現在更新時刻 <= self.最終更新時刻 {
            return シェーダー変化結果::変化なし;
        }
        // 失敗時もmtimeを更新し、次の保存で再試行される形にする。
        self.最終更新時刻 = 現在更新時刻;

        match compile::頂点とフラグメントをコンパイルする(&self.エントリパス) {
            Ok(シェーダー) => シェーダー変化結果::成功 { シェーダー },
            Err(メッセージ) => シェーダー変化結果::失敗 { メッセージ },
        }
    }
}
