//! パーセンタイル値の定義を、どの層から見ても1つに保つ層。
//!
//! ここが在るのは、同じ「95パーセンタイル値」がクレートごとに別の式で求められていたためである。
//! 描画のGPU時間・フレーム間隔・輝度の分布・遠景の沈み・検収の計測が、それぞれ自前の添字の式を持っており、
//! 同じ標本に対して違う添字を選んでいた。写しが散ると、報告に並んだ数どうしを比べられなくなる。
//!
//! 順位(50・95・99など)を識別子の文字列へ焼き込まず、値として受け渡すのもこの層の役目である。
//! 焼き込むと、順位を1つ足すたびに型のフィールドと関数が1つずつ増え、99.9のように整数でない順位を指せない。

mod ascending_samples;
mod percentile_rank;
mod rank_index_rule;
mod total_order;

#[cfg(test)]
mod statistics_tests;

pub use ascending_samples::昇順に並べた標本列;
pub use percentile_rank::パーセンタイル順位;
pub use rank_index_rule::順位から添字を選ぶ規則;
pub use total_order::全順序で比べられる値;
