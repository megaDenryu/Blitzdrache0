//! 高さ格子: 地形チャンクのソース形式。世界全体にひとつだけある大域サンプル格子から、チャンク1つ分の正方窓を切り出した高さの標本である。
//! 窓は隣接チャンクと重なる縁を持ち、境界の格子点でも中央差分の法線を同じ標本から計算できる。これにより隣接チャンクの境界の高さと法線が計算だけで一致する。
//! 参照: `_doc/設計/地形とカメラ相対描画.md`「地形の表現」

mod cutout;
mod error;
mod grid;
mod read;
mod specification;
mod write;

pub use cutout::高さ格子を切り出す;
pub use error::高さ格子エラー;
pub use grid::高さ格子;
pub use read::高さ格子を読み込む;
pub use specification::高さ格子諸元;
pub use write::高さ格子を格納する;

/// ファイル先頭の固定識別値。別形式のバイト列を高さの並びとして読み進めないための識別子である。
pub(crate) const 固定識別値: [u8; 8] = *b"BLITZHGT";

pub(crate) const 形式版: u32 = 1;

/// 固定識別値8 + 形式版4 + 辺分割数2 + 重なり幅1 + 一辺メートル4 の合計。
/// 幅の違う整数を並べて詰め物を置かないのは、詰め物が「未設定の値」として形式へ入り込むためである。
pub(crate) const ヘッダー長: usize = 19;
