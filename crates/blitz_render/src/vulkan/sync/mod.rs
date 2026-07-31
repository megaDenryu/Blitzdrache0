//! 進行中フレーム2のための同期プリミティブ。
//! `フレーム同期`（フェンス・取得セマフォ、フレームごとに多重化）と
//! `提示同期`（提示セマフォ、スワップチェーン画像ごと）に分かれる。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断13」。

mod frame_slot_index;
mod frame_sync;
mod present_sync;

pub(crate) use frame_slot_index::フレームスロット添字;
pub(crate) use frame_sync::フレーム同期;
pub(crate) use present_sync::提示同期;

/// 同時に処理を進めるフレーム数。CPUが次フレームを準備する間にGPUが前フレームを描く。
pub(crate) const 進行中フレーム数: usize = 2;
