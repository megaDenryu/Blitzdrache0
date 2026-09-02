//! 剛体のGPUレイアウトの正本と、その写しの対応。4つのバッファの1件のバイト数を、書き手(`blitz_sim::gpu_layout::rigid`)と、
//! 往復の検査が持つ写しで突き合わせる。GPUの写し(`blitz_render`の読み手とシェーダー)はIssue #43で足し、そのときこの表へ組を足す。
//! 正本の値だけを変えても検査の写しが追随しない限りここが落ち、契約の変更が検査の変更と対になる。

use super::定数の組;

const 正本: &str = "crates/blitz_sim/src/gpu_layout/rigid/mod.rs";
const 写し: &str = "crates/blitz_sim/src/gpu_layout/rigid/rigid_tests.rs";

pub(super) const 定数一覧: [定数の組; 4] = [
    定数の組 {
        正本パス: 正本,
        正本の前置き: "pub const 剛体運動状態1件のバイト数: usize = ",
        写しパス: 写し,
        写しの前置き: "const 剛体運動状態1件のバイト数の写し: usize = ",
    },
    定数の組 {
        正本パス: 正本,
        正本の前置き: "pub const 剛体前状態1件のバイト数: usize = ",
        写しパス: 写し,
        写しの前置き: "const 剛体前状態1件のバイト数の写し: usize = ",
    },
    定数の組 {
        正本パス: 正本,
        正本の前置き: "pub const 剛体質量特性1件のバイト数: usize = ",
        写しパス: 写し,
        写しの前置き: "const 剛体質量特性1件のバイト数の写し: usize = ",
    },
    定数の組 {
        正本パス: 正本,
        正本の前置き: "pub const 剛体実行状態1件のバイト数: usize = ",
        写しパス: 写し,
        写しの前置き: "const 剛体実行状態1件のバイト数の写し: usize = ",
    },
];
