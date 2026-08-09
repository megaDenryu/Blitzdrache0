//! 縮小段のオフラインの生成。sRGBの符号値と線形値の対応表、1段を半分に縮める工程、
//! 原寸から1x1までの全段を並べる工程に分かれる。
//! ブロック圧縮では段ごとに符号化するため、段の画素はアセットコンパイラが作る。
//! 参照: `_doc/設計/テクスチャのブロック圧縮と縮小段生成.md`

mod half_reduction;
mod level_chain;
mod srgb_linear_table;

#[cfg(test)]
mod reduction_tests;
#[cfg(test)]
mod srgb_table_tests;

pub use half_reduction::srgbの色として縦横を半分に縮める;
pub use level_chain::srgbの色として縮小段の連なりを作る;
