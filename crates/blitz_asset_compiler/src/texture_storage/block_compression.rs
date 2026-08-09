//! BC1の符号化。ブロック内の色から端点を選ぶ工程、端点をRGB565へ量子化する工程、
//! 端点が張る4色から索引を割り当てる工程、1ブロックを8バイトへ詰める工程、
//! 段をブロックへ区切って走査する工程に分かれる。
//! 復号はGPUが行うため本番の経路には無く、参照の復号器は検査だけが呼ぶ。
//! 参照: `_doc/設計/テクスチャのブロック圧縮と縮小段生成.md`

mod block_encode;
mod block_scan;
mod endpoint_search;
mod palette;
#[cfg(test)]
mod reference_decode;
mod rgb565;

#[cfg(test)]
mod encode_test_fixture;
#[cfg(test)]
mod encode_tests;
#[cfg(test)]
mod roundtrip_tests;

pub use block_scan::rgba8の縮小段をbc1のバイト列へ符号化する;
