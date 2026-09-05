//! blitz_collisionの形と接触の型ごとの分量の一覧。1行が1つの型の現状を写す。
//!
//! 注意: この一覧への追加は、閾値を超える型を新しく作ってよいという意味ではない。
//! 値を増やす向きへ書き換えてよいのは、増加が設計上避けられないと判断したときだけである。
//! 並びは根からのパスと型名をこの順で比べた文字コード順である。

use super::super::{区画の一覧, 台帳の行};

const モジュールの根: &str = "crates/blitz_collision/src";

const 行一覧: [台帳の行; 9] = [
    台帳の行::構造体("capsule/sweep_solver.rs", "カプセルどうしの接触の求解", 5, 3, 27),
    台帳の行::構造体("dynamic_index/tree.rs", "動く形の境界箱の木", 6, 2, 19),
    台帳の行::構造体("height_field/capsule_sweep/contact_solver.rs", "カプセルと三角形の接触の求解", 9, 2, 31),
    台帳の行::構造体("height_field/grid_origin_displacement.rs", "高さ場の中の変位", 2, 3, 16),
    台帳の行::構造体("oriented_box/sweep_solver.rs", "カプセルと直方体の接触の求解", 8, 2, 28),
    台帳の行::構造体("shape/global_axis_aligned_box.rs", "大域の軸平行の直方体", 2, 2, 16),
    台帳の行::構造体("shape/local_displacement.rs", "形の局所座標の変位", 3, 3, 21),
    台帳の行::構造体("shape/oriented_box_double.rs", "任意姿勢の直方体の倍精度の幾何", 8, 3, 25),
    台帳の行::構造体("triangle/sweep_solver.rs", "カプセルと三角形の接触の求解", 8, 2, 28),
];

pub fn 一覧() -> 区画の一覧 {
    区画の一覧::生成する(モジュールの根, &行一覧, file!())
}
