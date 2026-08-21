//! チャンク目録の内容だけが返す復元失敗。

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum チャンク目録実行時形式エラー {
    #[error("座標のu64表現{0}が重複している")]
    座標重複(u64),
    #[error("一辺が正の有限値でなかった")]
    一辺不正,
}
