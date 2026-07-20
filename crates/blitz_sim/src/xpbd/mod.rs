//! CPU参照計算(判断55): XPBD拘束解決の数式をテストとシェーダー仕様の両方に使う。

mod distance_projection;
#[cfg(test)]
mod distance_projection_tests;

pub use distance_projection::距離拘束を射影する;
