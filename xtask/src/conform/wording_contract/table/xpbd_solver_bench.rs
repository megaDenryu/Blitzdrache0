//! XPBDの並列方式の計測の報告の行の見出しと、一刻みの合計の区間名。
//! 見出しは行の種類を決める語であり、読む側は見出しで行を選んでから鍵の値を読む。片側だけ動くと、その種類の行が
//! 1本も読まれないまま「区間が無い」という失敗になる。区間名は積む側(blitz_render)と読む側(xtask)がアプリの報告を介して結ぶ。

use super::綴りの契約;

const 出す側: &str = "crates/blitz_app/src/reports/xpbd_solver_bench/lines.rs";
const 読む側: &str = "xtask/src/xpbd_solver_bench/parse/observation.rs";
const 両側: &[&str] = &[出す側, 読む側];

pub(super) const 綴り一覧: [綴りの契約; 7] = [
    綴りの契約 {
        綴り: "XPBD並列方式計測",
        現れるファイル一覧: 両側,
    },
    綴りの契約 {
        綴り: "XPBD検証",
        現れるファイル一覧: 両側,
    },
    綴りの契約 {
        綴り: "XPBD再現性",
        現れるファイル一覧: 両側,
    },
    綴りの契約 {
        綴り: "XPBD収束",
        現れるファイル一覧: 両側,
    },
    綴りの契約 {
        綴り: "XPBDのCPU参照との差",
        現れるファイル一覧: 両側,
    },
    綴りの契約 {
        綴り: "XPBD資源",
        現れるファイル一覧: 両側,
    },
    綴りの契約 {
        綴り: "XPBDの一刻みの合計",
        現れるファイル一覧: &[
            "crates/blitz_render/src/vulkan/xpbd_bench/pass_names.rs",
            "xtask/src/xpbd_solver_bench/intervals.rs",
        ],
    },
];
