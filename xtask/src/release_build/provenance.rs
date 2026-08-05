//! 構築した実行ファイルの由来。担当するのは、計測に使ったバイナリがどの版から作られたかを1行で言える形に
//! まとめることだけである。計測の値も条件も知らない。
//!
//! これが要るのは、計測の値だけを見ても「どの版で測ったか」が後から分からないためである。実行ファイルを直に
//! 叩く入口は、事前構築を挟んでいても、その構築が本当に効いたかをログから確かめる術を持たなかった。
//! 2026-08-06に、古い実行ファイルで走った計測の値が新しい契約と食い違う事態が実際に起きた。
//!
//! ハッシュでなく更新時刻と大きさを採るのは、リリースの実行ファイルが数十メガバイトあり、計測のたびに全体を
//! 読み直す費用に見合わないためである。構築の直後に採るため、更新時刻が構築より古ければ構築が効いていない。

use std::path::Path;
use std::process::Command;
use std::time::UNIX_EPOCH;

/// 計測に使った実行ファイルの出どころ。
pub struct 構築の由来 {
    コミット: String,
    作業ツリーが汚れているか: bool,
    実行ファイルの更新時刻: String,
    実行ファイルのバイト数: u64,
}

impl 構築の由来 {
    /// 構築の直後に採る。gitが読めない環境でも計測は止めず、読めなかったことを値として残す。
    pub fn 採る(実行ファイル: &Path) -> Self {
        let (更新時刻, バイト数) = 実行ファイルの状態(実行ファイル);
        Self {
            コミット: gitの出力("rev-parse", &["--short", "HEAD"]).unwrap_or_else(|| "読めない".to_string()),
            作業ツリーが汚れているか: gitの出力("status", &["--porcelain"]).is_none_or(|出力| !出力.is_empty()),
            実行ファイルの更新時刻: 更新時刻,
            実行ファイルのバイト数: バイト数,
        }
    }

    /// ログとtsvへ同じ綴りで残す1行。由来の確認がこの行だけで閉じる。
    pub fn 一行にする(&self) -> String {
        format!(
            "コミット={} 作業ツリー={} 実行ファイルの更新時刻={} バイト数={}",
            self.コミット,
            if self.作業ツリーが汚れているか {
                "変更あり"
            } else {
                "変更なし"
            },
            self.実行ファイルの更新時刻,
            self.実行ファイルのバイト数
        )
    }
}

/// 実行ファイルの更新時刻(紀元からの秒)とバイト数。読めなければ読めなかったことを残す。
fn 実行ファイルの状態(実行ファイル: &Path) -> (String, u64) {
    let Ok(情報) = std::fs::metadata(実行ファイル) else {
        return ("読めない".to_string(), 0);
    };
    let 更新時刻 = 情報
        .modified()
        .ok()
        .and_then(|時刻| 時刻.duration_since(UNIX_EPOCH).ok())
        .map_or_else(|| "読めない".to_string(), |経過| 経過.as_secs().to_string());
    (更新時刻, 情報.len())
}

fn gitの出力(副命令: &str, 引数一覧: &[&str]) -> Option<String> {
    let 出力 = Command::new("git").arg(副命令).args(引数一覧).output().ok()?;
    if !出力.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&出力.stdout).trim().to_string())
}
