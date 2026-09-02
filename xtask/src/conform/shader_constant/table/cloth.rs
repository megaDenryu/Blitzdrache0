//! 布シミュレーションの正本と、その写しの対応。計算の発行が刻む班のスレッド数を、拘束の工程のシェーダーの
//! `threadsPerGroup`と突き合わせる。値がずれると色の区間の端の拘束が処理されないまま絵も報告も出るため、ここが機械的に見る。
//! 拘束1件のバイト数は、書き手(`blitz_sim::gpu_layout::xpbd`)と、blitz_simへ依存できないため値を持ち直した読み手
//! (`blitz_render::cloth_material`)を突き合わせる。ずれると拘束の内容の検証が別の位置を読み、検証が通ったまま絵が壊れる。

use super::定数の組;

pub(super) const 定数一覧: [定数の組; 2] = [
    定数の組 {
        正本パス: "crates/blitz_render/src/vulkan/frame/record/cloth_passes/dispatch.rs",
        正本の前置き: "const 班のスレッド数: u32 = ",
        写しパス: "shaders/cloth_constraint.slang",
        写しの前置き: "static const uint threadsPerGroup = ",
    },
    定数の組 {
        正本パス: "crates/blitz_sim/src/gpu_layout/xpbd.rs",
        正本の前置き: "pub const 拘束1件のバイト数: usize = ",
        写しパス: "crates/blitz_render/src/cloth_material/coloring.rs",
        写しの前置き: "pub(super) const 拘束1件のバイト数: usize = ",
    },
];
