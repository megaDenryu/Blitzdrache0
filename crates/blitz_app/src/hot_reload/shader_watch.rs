//! シェーダーソースのmtime監視。変化検知でslangcを子プロセス実行し再コンパイルする。

use std::path::PathBuf;
use std::time::SystemTime;

use blitz_render::シェーダー一式;

use super::{compile, mtime};

/// シェーダー再コンパイルの結果。
pub(super) enum シェーダー変化結果 {
    変化なし,
    成功 { シェーダー: シェーダー一式 },
    失敗 { メッセージ: String },
}

pub(super) struct シェーダー監視状態 {
    パス: PathBuf,
    最終更新時刻: SystemTime,
}

pub(super) fn 構築する(監視パス: PathBuf) -> Option<シェーダー監視状態> {
    mtime::取得する(&監視パス)
        .ok()
        .map(|最終更新時刻| シェーダー監視状態 { パス: 監視パス, 最終更新時刻 })
}

impl シェーダー監視状態 {
    pub(super) fn 変化を確認する(&mut self) -> シェーダー変化結果 {
        let Ok(現在更新時刻) = mtime::取得する(&self.パス) else {
            // 保存の途中でファイルが一時的に読めない等は無視し、次回の確認に委ねる。
            return シェーダー変化結果::変化なし;
        };
        if 現在更新時刻 <= self.最終更新時刻 {
            return シェーダー変化結果::変化なし;
        }
        // 失敗時もmtimeを更新し、次の保存で再試行される形にする。
        self.最終更新時刻 = 現在更新時刻;

        match compile::頂点とフラグメントをコンパイルする(&self.パス) {
            Ok(シェーダー) => シェーダー変化結果::成功 { シェーダー },
            Err(メッセージ) => シェーダー変化結果::失敗 { メッセージ },
        }
    }
}
