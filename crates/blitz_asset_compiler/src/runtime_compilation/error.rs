//! 実行時アセットの公開ライブラリ入口が、直すべきソース不正と実行環境の入出力失敗を区別して返す。

#[derive(Debug, thiserror::Error)]
pub enum 実行時アセットのコンパイルエラー {
    #[error("ソースアセットが不正である: {0}")]
    ソース不正(String),
    #[error("実行時アセットの入出力に失敗した: {0}")]
    入出力(String),
}

impl 実行時アセットのコンパイルエラー {
    pub(crate) fn ソースが不正(誤り: impl ToString) -> Self {
        Self::ソース不正(誤り.to_string())
    }

    pub(crate) fn 入出力に失敗(誤り: impl ToString) -> Self {
        Self::入出力(誤り.to_string())
    }
}
