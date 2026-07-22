//! カタログに格納する実行時容量の見積もり情報。

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct アセットメタデータ {
    pub 頂点数: u64,
    pub インデックス数: u64,
    pub テクスチャバイト数: u64,
}
