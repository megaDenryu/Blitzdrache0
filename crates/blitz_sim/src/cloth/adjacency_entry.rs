//! 隣接拘束エントリ: 1粒子から見た1本の隣接拘束(GPUのgather方式反復用。判断49追記)。
//! 参照: 開発スレッド「M9のGPU側実装の詳細設計」。

/// `相手粒子添字`が`空き添字`ならスロットは未使用(粒子ごと最大8本に満たない場合の埋め)。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 隣接拘束エントリ {
    pub 相手粒子添字: u32,
    pub 静止長: f32,
}

/// 空きスロットを表す相手粒子添字(0xFFFFFFFF)。
pub const 空き添字: u32 = u32::MAX;

impl 隣接拘束エントリ {
    pub fn 空き() -> Self {
        Self {
            相手粒子添字: 空き添字,
            静止長: 0.0,
        }
    }
}
