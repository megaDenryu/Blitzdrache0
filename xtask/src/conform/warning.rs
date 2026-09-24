//! 規約適合検査の警告1件を表す値オブジェクト。警告とは、`cargo xtask conform` が表示するが終了コードを変えない報告のことである。
//! 違反と別の型にするのは、警告を違反の一覧へ混ぜて終了コードを変える呼び出しを署名の上で成立させないためである。表示の形(ファイル・行・文言)は違反と同じにする。

use std::fmt;
use std::path::PathBuf;

pub struct 警告 {
    pub パス: PathBuf,
    pub 行番号: usize,
    pub 説明: String,
}

impl 警告 {
    pub const fn 行単位(パス: PathBuf, 行番号: usize, 説明: String) -> Self {
        Self { パス, 行番号, 説明 }
    }
}

impl fmt::Display for 警告 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: {}", self.パス.display(), self.行番号, self.説明)
    }
}
