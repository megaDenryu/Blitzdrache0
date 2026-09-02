//! 布シミュレーションの正本と、その写しの対応。計算の発行が刻む班のスレッド数を、拘束の工程のシェーダーの
//! `threadsPerGroup`と突き合わせる。値がずれると色の区間の端の拘束が処理されないまま絵も報告も出るため、ここが機械的に見る。

use super::定数の組;

pub(super) const 定数一覧: [定数の組; 1] = [定数の組 {
    正本パス: "crates/blitz_render/src/vulkan/frame/record/cloth_passes/dispatch.rs",
    正本の前置き: "const 班のスレッド数: u32 = ",
    写しパス: "shaders/cloth_constraint.slang",
    写しの前置き: "static const uint threadsPerGroup = ",
}];
