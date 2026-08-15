//! 散布した個体数の合計を出す行の綴り。
//!
//! この行はアセットコンパイラが焼き上がりのたびに出し、生成の検収が「散布が効いたこと」を確かめるために読む。
//! 散布が効いたかは生成物の中身に現れない。チャンクのファイルは散布があってもなくても同じ形式であり、
//! 個体数の合計が唯一の証拠である。綴りがずれると検収は行を見つけられず、散布を1体も置かない実行が緑で通る。

use super::綴りの契約;

pub(super) const 綴り一覧: [綴りの契約; 1] = [綴りの契約 {
    綴り: "個体数の合計=",
    現れるファイル一覧: &[
        "crates/blitz_asset_compiler/examples/compile_assets/instance_tally.rs",
        "xtask/src/game_fox_tour/map_generation_check/compile_report.rs",
    ],
}];
