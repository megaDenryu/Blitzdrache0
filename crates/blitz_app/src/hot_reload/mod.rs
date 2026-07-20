//! シェーダーホットリロード: stdのみのmtimeポーリング。新規依存クレートは追加しない。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断7」。

mod compile;
mod mtime;
mod slangc;

use std::path::PathBuf;
use std::time::{Duration, Instant};

use blitz_render::シェーダー一式;

const 確認間隔: Duration = Duration::from_millis(500);

/// `ホットリローダー::確認する` の結果。
pub(crate) enum ホットリロード結果 {
    /// 前回確認から間隔未満、または監視対象に変化が無かった。
    変化なし,
    /// 変化を検知し、再コンパイルに成功した。
    再コンパイル成功 { シェーダー: シェーダー一式 },
    /// 変化を検知したが、再コンパイルに失敗した（旧パイプライン継続）。
    再コンパイル失敗 { メッセージ: String },
}

struct 監視状態 {
    パス: PathBuf,
    最終更新時刻: std::time::SystemTime,
}

/// シェーダーソースのmtimeを約0.5秒間隔でポーリングし、変化を検知したら
/// slangcで両エントリを再コンパイルする。監視対象パスが存在しなければ監視を無効化する。
pub(crate) struct ホットリローダー {
    監視対象: Option<監視状態>,
    前回確認時刻: Instant,
}

impl ホットリローダー {
    pub(crate) fn 生成する(監視パス: PathBuf) -> Self {
        let 監視対象 = mtime::取得する(&監視パス)
            .ok()
            .map(|最終更新時刻| 監視状態 { パス: 監視パス, 最終更新時刻 });
        Self {
            監視対象,
            前回確認時刻: Instant::now(),
        }
    }

    pub(crate) fn 確認する(&mut self) -> ホットリロード結果 {
        let Some(監視状態) = &mut self.監視対象 else {
            return ホットリロード結果::変化なし;
        };

        let 今 = Instant::now();
        if 今.duration_since(self.前回確認時刻) < 確認間隔 {
            return ホットリロード結果::変化なし;
        }
        self.前回確認時刻 = 今;

        let Ok(現在更新時刻) = mtime::取得する(&監視状態.パス) else {
            // 保存の途中でファイルが一時的に読めない等は無視し、次回の確認に委ねる。
            return ホットリロード結果::変化なし;
        };
        if 現在更新時刻 <= 監視状態.最終更新時刻 {
            return ホットリロード結果::変化なし;
        }
        // 判断7: 失敗時もmtimeを更新し、次の保存で再試行される形にする。
        監視状態.最終更新時刻 = 現在更新時刻;

        match compile::頂点とフラグメントをコンパイルする(&監視状態.パス) {
            Ok(シェーダー) => ホットリロード結果::再コンパイル成功 { シェーダー },
            Err(メッセージ) => ホットリロード結果::再コンパイル失敗 { メッセージ },
        }
    }
}
