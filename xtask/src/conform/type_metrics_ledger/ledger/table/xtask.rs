//! xtaskの検収と計測の型ごとの分量の一覧。1行が1つの型の現状を写す。
//!
//! 注意: この一覧への追加は、閾値を超える型を新しく作ってよいという意味ではない。
//! 値を増やす向きへ書き換えてよいのは、増加が設計上避けられないと判断したときだけである。
//! 並びは根からのパスと型名をこの順で比べた文字コード順である。

use super::super::{区画の一覧, 台帳の行};

const モジュールの根: &str = "xtask/src";

const 行一覧: [台帳の行; 10] = [
    台帳の行::構造体("acceptance/exit_report.rs", "終了時報告", 2, 3, 17),
    台帳の行::構造体("acceptance/judgment_name.rs", "判定の名前", 3, 0, 17),
    台帳の行::構造体("auto_exposure/parse.rs", "自動露出の報告", 0, 16, 0),
    台帳の行::構造体("cloth_xpbd_reference/parse.rs", "参照比較の観測", 1, 17, 1),
    台帳の行::列挙("distant_view/plan.rs", "実行の別", 1, 22, 1),
    台帳の行::構造体("game_fox_tour/map_generation_check/check_root.rs", "検収用のルート", 3, 0, 18),
    台帳の行::構造体("hdr_luminance/statistics.rs", "輝度統計", 0, 11, 0),
    台帳の行::構造体("large_world_bench/arguments.rs", "大規模世界の計測指定", 1, 15, 1),
    台帳の行::構造体("report_parse.rs", "計数報告", 0, 12, 0),
    台帳の行::構造体("xpbd_solver_bench/record.rs", "方式の観測", 0, 21, 0),
];

pub fn 一覧() -> 区画の一覧 {
    区画の一覧::生成する(モジュールの根, &行一覧, file!())
}
