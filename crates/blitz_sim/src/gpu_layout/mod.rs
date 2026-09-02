//! GPUバッファレイアウト定義(判断52・54): std430前提のバイト列組み立て。
//! blitz_renderのslangシェーダーがこの仕様に合わせて読む(親の実装)。

mod adjacency_bytes;
mod constraint_bytes;
#[cfg(test)]
mod gpu_layout_tests;
mod particle_bytes;
mod surface_bytes;
#[cfg(test)]
mod surface_bytes_tests;
pub mod xpbd;

pub use adjacency_bytes::隣接拘束バイト列にする;
pub use constraint_bytes::拘束バイト列にする;
pub use particle_bytes::粒子バイト列にする;
pub use surface_bytes::表面流バイト列にする;
