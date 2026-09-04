//! `blitz_render`のVulkan層と描画の組み立て層を除いた未是正の自由関数の一覧。件数が少ないため1つの表にまとめる。
//!
//! 注意: この一覧への追加は禁止する。減らす方向にのみ動かす。削除できるのは、その工程が自分の触るものだけを
//! 名前の付いた引数で受け取る形へ直したときか、操作を親の型のメソッドへ移したときだけである。

use super::super::{区画の一覧, 未是正の自由関数};

const 項目一覧: [未是正の自由関数; 7] = [
    未是正の自由関数::生成する("atmosphere/bake/aerial_lut.rs", "区間長を求める", "空中遠近ボリュームの材料"),
    未是正の自由関数::生成する("atmosphere/bake/aerial_lut.rs", "幾何を組む", "空中遠近ボリュームの材料"),
    未是正の自由関数::生成する("atmosphere/integration/multiscatter_step.rs", "区間を積む", "区間の条件"),
    未是正の自由関数::生成する("atmosphere/integration/skyview_step.rs", "区間を積む", "視線の幾何"),
    未是正の自由関数::生成する("distant_environment/analytic_input.rs", "鏡面畳込みが妥当か", "鏡面畳込みの解析入力"),
    未是正の自由関数::生成する("frame_input/sky_input/validation.rs", "確かめる", "空入力の材料"),
    未是正の自由関数::生成する(
        "xpbd_solver_bench_probe/mod.rs",
        "xpbdの並列方式をgpuで走らせて読み戻す",
        "XPBD計測の条件",
    ),
];

pub fn 一覧() -> 区画の一覧 {
    区画の一覧::生成する("crates/blitz_render/src", &項目一覧)
}
