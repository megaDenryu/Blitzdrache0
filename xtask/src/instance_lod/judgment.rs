//! 個体別LODの検収の判定。1回ぶんの実行結果をここが所有し、何を見るかは2つの子が持つ。
//! `tier`は段の立ち方(同じ群の中で2段以上が同時に立つこと・段の違いが遠景の画素に出て近景は変わらないこと・
//! ヒステリシス帯の内側の往復で段が振動しないこと)を、`resource`は段が切り替わってもGPU確保とディスク読込が
//! 動かないことを見る。

mod resource;
mod tier;

pub(super) use resource::確保と読込が動かないことを検査する;
pub(super) use tier::{
    段が同時に立つことを検査する, 段が振動しないことを検査する, 段の違いが絵に出ることを検査する
};

use crate::acceptance::読み戻し画像;
use crate::report_parse::計数報告;

pub(super) struct 実行 {
    pub(super) 名前: String,
    pub(super) 画像: 読み戻し画像,
    pub(super) 計数: 計数報告,
}
