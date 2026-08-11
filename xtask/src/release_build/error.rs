//! 計測用のリリースビルドが返す型付きエラー。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//!
//! xtaskは外部のクレートへ依存しないため、thiserrorを使わず手書きのenumと`Display`で書く。
//! 様式は`acceptance/error.rs`に倣う。
//!
//! 「起こせなかった」と「失敗して終わった」を別の枝で持つのは、直す側が見る場所が違うためである。
//! 前者はcargoが実行環境に無く、後者はコードが通っていない。

#[derive(Debug)]
pub enum 計測用の構築の破れ {
    構築の道具cargoを起こせなかった { コマンド名: &'static str, 誤り: std::io::Error },
    構築が失敗して終わった { コマンド名: &'static str, 終了状態: String },
}

impl std::error::Error for 計測用の構築の破れ {}

impl std::fmt::Display for 計測用の構築の破れ {
    fn fmt(&self, 書き手: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::構築の道具cargoを起こせなかった { コマンド名, 誤り } => {
                write!(書き手, "{コマンド名}のリリースビルドでcargoを起動できなかった: {誤り}")
            }
            Self::構築が失敗して終わった {
                コマンド名, 終了状態
            } => write!(
                書き手,
                "{コマンド名}のリリースビルドが終了コード{終了状態}で失敗した(直そうとした版で測れないためここで止める)"
            ),
        }
    }
}
