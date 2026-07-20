//! ピクセル読み戻し用のホスト可視バッファ確保と、読み戻し画像への変換。
//! GPU同期(フェンス待機)を伴うためスモーク用途と文書化する。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断9」。

mod buffer;
mod memory;
mod read;

pub(crate) use buffer::読み戻しバッファ;
pub(crate) use read::読み取る;
