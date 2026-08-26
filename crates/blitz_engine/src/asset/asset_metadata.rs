//! カタログに格納する実行時容量の見積もり情報。

use super::height_field_sample_count::高さ場の標本数;

/// `個体数`を頂点数と別に数えるのは、個体ごとの配置と描画用変換がRAMとVRAMを占めるためである。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct アセットメタデータ {
    pub 頂点数: u64,
    pub インデックス数: u64,
    pub テクスチャ格納バイト数: u64, // 格納形式で数えた値であり、GPUへ載る実バイト数と一致する
    pub 個体数: u64, // インスタンス群の個体の総数。持たないアセットでは0
    pub 高さ場の標本数: 高さ場の標本数, // 格子が持つ標本の総数。高さ場でないアセットでは0
}
