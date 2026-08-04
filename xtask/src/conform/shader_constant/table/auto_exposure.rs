//! 自動露出の正本と、その写しの対応。触れるのは相対輝度の3成分の重みだけである。
//!
//! 写しがxtask自身の中にあるのは、範囲の制定に使う実測の入口(`cargo xtask hdr-luminance`)が
//! 外部依存を持たない方針を保ち、blitz_engineを読めないためである。重みが片方だけ動くと、
//! 制定したヒストグラムの範囲が実測の分布と対応しなくなる。

use super::定数の組;

pub(super) const 定数一覧: [定数の組; 3] = [
    定数の組 {
        正本パス: "crates/blitz_engine/src/auto_exposure/luminance.rs",
        正本の前置き: "pub const 赤の重み: f32 = ",
        写しパス: "xtask/src/hdr_luminance/statistics.rs",
        写しの前置き: "const 赤の重み: f64 = ",
    },
    定数の組 {
        正本パス: "crates/blitz_engine/src/auto_exposure/luminance.rs",
        正本の前置き: "pub const 緑の重み: f32 = ",
        写しパス: "xtask/src/hdr_luminance/statistics.rs",
        写しの前置き: "const 緑の重み: f64 = ",
    },
    定数の組 {
        正本パス: "crates/blitz_engine/src/auto_exposure/luminance.rs",
        正本の前置き: "pub const 青の重み: f32 = ",
        写しパス: "xtask/src/hdr_luminance/statistics.rs",
        写しの前置き: "const 青の重み: f64 = ",
    },
];
