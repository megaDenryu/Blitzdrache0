//! 材質集合の生成が返す失敗型。

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum 材質集合エラー {
    #[error("材質集合に材質が1件も存在しない")]
    材質なし,
    #[error("材質集合で材質スロットID {番号}が重複している")]
    スロットID重複 { 番号: u32 },
}
