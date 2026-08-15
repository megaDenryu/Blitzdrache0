//! 影の距離区分の分け方を出す行の綴り。
//!
//! この行はアプリが起動時に必ず出し、継ぎ目の検収が「構図が要る分け方で走ったこと」を確かめるために読む。
//! 再配分が効いたかは絵から確かめられない。実用分割のままでも4距離区分は現れ境界の段差も出るため、
//! 判定はどちらでも通ってしまう。綴りがずれると検収は行を見つけられず、再配分前の絵を検収したまま緑になる。

use super::綴りの契約;

pub(super) const 綴り一覧: [綴りの契約; 2] = [
    綴りの契約 {
        綴り: "影の距離区分の分け方:",
        現れるファイル一覧: &[
            "crates/blitz_app/src/reports/shadow_band_assignment.rs",
            "xtask/src/csm_seam/band_assignment_line.rs",
        ],
    },
    綴りの契約 {
        綴り: "明示境界",
        現れるファイル一覧: &[
            "crates/blitz_app/src/reports/shadow_band_assignment.rs",
            "xtask/src/csm_seam/band_assignment_line.rs",
        ],
    },
];
