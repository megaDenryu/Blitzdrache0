//! 規約適合検査の違反1件を表す値オブジェクト。

use std::fmt;
use std::path::PathBuf;

pub struct 違反 {
    pub パス: PathBuf,
    pub 行番号: Option<usize>,
    pub 説明: String,
}

impl 違反 {
    pub fn ファイル単位(パス: PathBuf, 説明: String) -> Self {
        Self {
            パス, 行番号: None, 説明
        }
    }

    pub fn 行単位(パス: PathBuf, 行番号: usize, 説明: String) -> Self {
        Self {
            パス,
            行番号: Some(行番号),
            説明,
        }
    }
}

impl fmt::Display for 違反 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.行番号 {
            Some(行番号) => write!(f, "{}:{}: {}", self.パス.display(), 行番号, self.説明),
            None => write!(f, "{}: {}", self.パス.display(), self.説明),
        }
    }
}
