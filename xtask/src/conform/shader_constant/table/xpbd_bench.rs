//! XPBDの並列方式の計測の正本と、その写しの対応。計算の発行が刻む班のスレッド数を、工程ごとのシェーダー4ファイルの
//! `threadsPerGroup`と突き合わせる。値がずれると端の点や拘束が処理されないまま絵も報告も出るため、ここが機械的に見る。

use super::定数の組;

const 班の正本: &str = "crates/blitz_render/src/vulkan/xpbd_bench/passes/dispatch.rs";
const 班の前置き: &str = "const 班のスレッド数: u32 = ";
const スレッド数の写しの前置き: &str = "static const uint threadsPerGroup = ";

pub(super) const 定数一覧: [定数の組; 4] = [
    定数の組 {
        正本パス: 班の正本,
        正本の前置き: 班の前置き,
        写しパス: "shaders/xpbd_step.slang",
        写しの前置き: スレッド数の写しの前置き,
    },
    定数の組 {
        正本パス: 班の正本,
        正本の前置き: 班の前置き,
        写しパス: "shaders/xpbd_atomic.slang",
        写しの前置き: スレッド数の写しの前置き,
    },
    定数の組 {
        正本パス: 班の正本,
        正本の前置き: 班の前置き,
        写しパス: "shaders/xpbd_coloring.slang",
        写しの前置き: スレッド数の写しの前置き,
    },
    定数の組 {
        正本パス: 班の正本,
        正本の前置き: 班の前置き,
        写しパス: "shaders/xpbd_two_stage.slang",
        写しの前置き: スレッド数の写しの前置き,
    },
];
