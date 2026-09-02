//! GPUバッファレイアウト定義(判断52・54): std430前提のバイト列組み立て。
//! blitz_renderのslangシェーダーがこの仕様に合わせて読む(親の実装)。

#[cfg(test)]
mod gpu_layout_tests;
mod particle_bytes;
mod surface_bytes;
#[cfg(test)]
mod surface_bytes_tests;
pub mod xpbd;
#[cfg(test)]
mod xpbd_tests;

pub use particle_bytes::粒子バイト列にする;
pub use surface_bytes::表面流バイト列にする;
