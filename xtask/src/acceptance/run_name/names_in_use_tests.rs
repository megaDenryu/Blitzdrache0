//! いま使っている実行名の台帳と、それがすべて検証を通ることの回帰検査。呼ばれるのは`cargo test`だけである。
//!
//! 台帳を検査と一緒に置くのは、許可する文字の集合を狭める改定が、使っている名前を拒む形で退行しうるためである。
//! 拒む側の検査だけでは、集合を狭めすぎた退行を1つも捕まえられない。

#![allow(clippy::unwrap_used)]

use super::検収の実行名;

/// 実際に使っている実行名がすべて通ることを固定する。許可の集合を狭めすぎた退行をここが拾う。
#[test]
fn 使っている実行名はすべて通る() {
    for 綴り in [
        "flat",
        "post",
        "night",
        "night_cloth",
        "night_preceding",
        "contrast_rgba8",
        "contrast_bc1",
        "contrast_bc1_repeat",
        "helmet_rgba8",
        "helmet_bc1",
        "region_mask",
        "ground_mask",
        "dawn",
        "noon",
        "evening",
        "on",
        "off",
        "band_inside",
        "band_outside",
        "east",
        "west",
        "north",
        "south",
        "scene",
        "bands",
        "prop_wooden_crate",
        "terrain_origin_before",
        "night_x4_1",
        "night_shadow_x2",
        "hut_instrument_x2",
        "halt_a",
        "advance_b",
        "halt_a2",
        "camera_still",
        "camera_moving",
        "run_003",
        "coarse_A",
        "fine_A",
        "ao128",
    ] {
        assert!(検収の実行名::生成する(綴り).is_ok(), "使っている実行名{綴り}が拒まれた");
    }
}
