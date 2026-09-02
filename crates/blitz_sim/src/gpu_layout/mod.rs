//! GPUバッファレイアウト定義(判断52・54): std430前提のバイト列組み立て。
//! blitz_renderのslangシェーダーがこの仕様に合わせて読む(親の実装)。
//! 剛体の4つのバッファの契約(判断22)は`rigid`が持ち、ここから同じ名前で公開する。

#[cfg(test)]
mod gpu_layout_tests;
mod particle_bytes;
pub mod rigid;
mod surface_bytes;
#[cfg(test)]
mod surface_bytes_tests;
pub mod xpbd;
#[cfg(test)]
mod xpbd_tests;

pub use particle_bytes::粒子バイト列にする;
pub use surface_bytes::表面流バイト列にする;
