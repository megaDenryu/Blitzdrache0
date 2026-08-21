//! 散布した個体数を出す4つの行の綴り。
//!
//! これらの行はアセットコンパイラが焼き上がりのたびに出し、生成の検収が「散布が効いたこと」と
//! 「群の描画対象への載せ漏れが無いこと」を確かめるために読む。散布が効いたかは生成物の中身に現れない。
//! チャンクのファイルは散布があってもなくても同じ形式であり、個体数の合計が唯一の証拠である。
//! 綴りがずれると検収は行を見つけられず、散布を1体も置かない実行が緑で通る。
//!
//! 個体数の合計は実行時カタログの容量メタデータから数えた値、置いた個体の合計は配置列がインスタンス群になる直前で
//! 数えた値であり、検収は2つの一致を課す。据え置いたチャンク数は、その一致を課してよい実行かどうかを決める。
//!
//! 種類ごとの置いた個体の行は、合計では読めない木と岩の取り分を出す。1本が幹と枝と葉房の複数の部品になる散布が
//! 入ったため、合計だけでは木を1本増やすと何体増えるのかを外から読めない。

use super::綴りの契約;

pub(super) const 綴り一覧: [綴りの契約; 4] = [
    綴りの契約 {
        綴り: "個体数の合計=",
        現れるファイル一覧: &[
            "crates/blitz_asset_compiler/src/runtime_compilation/instance_tally.rs",
            "xtask/src/game_fox_tour/map_generation_check/compile_report.rs",
        ],
    },
    綴りの契約 {
        綴り: "種類ごとの置いた個体:",
        現れるファイル一覧: &[
            "crates/blitz_asset_compiler/src/runtime_compilation/placed_instance_tally.rs",
            "xtask/src/game_fox_tour/map_generation_check/tally_agreement.rs",
        ],
    },
    綴りの契約 {
        綴り: "置いた個体の合計=",
        現れるファイル一覧: &[
            "crates/blitz_asset_compiler/src/runtime_compilation/placed_instance_tally.rs",
            "xtask/src/game_fox_tour/map_generation_check/compile_report.rs",
        ],
    },
    綴りの契約 {
        綴り: "据え置いたチャンク数=",
        現れるファイル一覧: &[
            "crates/blitz_asset_compiler/src/generation_ledger/rebake_tally.rs",
            "xtask/src/game_fox_tour/map_generation_check/compile_report.rs",
        ],
    },
];
