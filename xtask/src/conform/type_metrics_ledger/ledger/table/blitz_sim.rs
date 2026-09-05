//! blitz_simの剛体と接触の型ごとの分量の一覧。1行が1つの型の現状を写す。
//!
//! 注意: この一覧への追加は、閾値を超える型を新しく作ってよいという意味ではない。
//! 値を増やす向きへ書き換えてよいのは、増加が設計上避けられないと判断したときだけである。
//! 並びは根からのパスと型名をこの順で比べた文字コード順である。

use super::super::{区画の一覧, 台帳の行};

const モジュールの根: &str = "crates/blitz_sim/src";

const 行一覧: [台帳の行; 6] = [
    台帳の行::構造体("contact/body_body_contact/mod.rs", "剛体と剛体の接触拘束", 4, 5, 26),
    台帳の行::構造体("contact/body_static_contact/mod.rs", "剛体と静的世界の接触拘束", 4, 5, 27),
    台帳の行::構造体("contact/normal_tangential_system/system.rs", "接触点集合の法線と接線の連立", 3, 3, 20),
    台帳の行::構造体("contact/scene/scene_settings.rs", "場面の設定", 0, 12, 0),
    台帳の行::構造体("contact/scene/substep_harness.rs", "一つの箱と静的な直方体の場面", 17, 16, 32),
    台帳の行::構造体("rigid_body/body/mod.rs", "剛体", 4, 5, 19),
];

pub fn 一覧() -> 区画の一覧 {
    区画の一覧::生成する(モジュールの根, &行一覧, file!())
}
