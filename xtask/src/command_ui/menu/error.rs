//! メニューの対話操作が失敗したことを表す型付きエラー。端末の生モード制御とキー入力読み取りは
//! すべてOSの入出力を経由するため、破れうる前提は「端末の制御に失敗した」の1枝しか無い。

#[derive(Debug)]
pub(crate) enum メニューの破れ {
    端末制御に失敗(std::io::Error),
}

impl std::fmt::Display for メニューの破れ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::端末制御に失敗(誤り) => write!(f, "端末の制御に失敗した: {誤り}"),
        }
    }
}

impl std::error::Error for メニューの破れ {}

impl From<std::io::Error> for メニューの破れ {
    fn from(誤り: std::io::Error) -> Self {
        Self::端末制御に失敗(誤り)
    }
}
