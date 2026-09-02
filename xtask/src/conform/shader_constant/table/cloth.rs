//! 布シミュレーションの正本と、その写しの対応。計算の発行が刻む班のスレッド数を、拘束の工程のシェーダーの
//! `threadsPerGroup`と突き合わせる。値がずれると色の区間の端の拘束が処理されないまま絵も報告も出るため、ここが機械的に見る。
//! 拘束1件のバイト数と目標拘束1件のバイト数と目標位置1件のバイト数と曲げ拘束1件のバイト数と粒子1件のバイト数は、書き手(`blitz_sim::gpu_layout`)と、
//! blitz_simへ依存できないため値を持ち直した読み手(`blitz_render::cloth_material`)を突き合わせる。ずれると内容の検証が別の位置を読み、検証が通ったまま絵が壊れる。
//! XPBDの退化の下限3つ(距離拘束の2点の距離・面の曲げ拘束の共有する辺の長さ・三角形の面積ベクトルの長さ)は、CPU正本(`blitz_sim::xpbd`)とGPUの写し
//! (`shaders/xpbd_projection.slang`・`shaders/xpbd_bending_projection.slang`)を突き合わせる。ずれると同じ形を片側だけ退化として扱う。突き合わせはどちらも同じ次元の裸の数で行う。
//! CPU正本は単位の型の定数の材料として裸の数の定数を名前付きで置き、GPUの写しは距離と辺を長さの2乗でなく長さで比べる形に揃えた(2乗の綴り1e-12と長さの綴り1e-6を検査が読み替える形は採らない)。

use super::定数の組;

pub(super) const 定数一覧: [定数の組; 10] = [
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
    定数の組 {
        正本パス: "crates/blitz_sim/src/gpu_layout/xpbd/target.rs",
        正本の前置き: "pub const 目標拘束1件のバイト数: usize = ",
        写しパス: "crates/blitz_render/src/cloth_material/target_check.rs",
        写しの前置き: "pub(super) const 目標拘束1件のバイト数: usize = ",
    },
    定数の組 {
        正本パス: "crates/blitz_sim/src/gpu_layout/xpbd/target.rs",
        正本の前置き: "pub const 目標位置1件のバイト数: usize = ",
        写しパス: "crates/blitz_render/src/cloth_material/target_check.rs",
        写しの前置き: "pub(super) const 目標位置1件のバイト数: usize = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/vulkan/frame/record/cloth_passes/dispatch.rs",
        正本の前置き: "const 班のスレッド数: u32 = ",
        写しパス: "shaders/cloth_bending.slang",
        写しの前置き: "static const uint threadsPerGroup = ",
    },
    定数の組 {
        正本パス: "crates/blitz_sim/src/gpu_layout/xpbd/bending.rs",
        正本の前置き: "pub const 曲げ拘束1件のバイト数: usize = ",
        写しパス: "crates/blitz_render/src/cloth_material/bending_check.rs",
        写しの前置き: "pub(super) const 曲げ拘束1件のバイト数: usize = ",
    },
    定数の組 {
        正本パス: "crates/blitz_sim/src/gpu_layout/particle_bytes.rs",
        正本の前置き: "const 粒子1件のバイト数: usize = ",
        写しパス: "crates/blitz_render/src/cloth_material/particle_check.rs",
        写しの前置き: "pub(super) const 粒子1件のバイト数: usize = ",
    },
    定数の組 {
        正本パス: "crates/blitz_sim/src/xpbd/distance_canonical_projection.rs",
        正本の前置き: "const 向きが定まる最小の距離のメートル: f32 = ",
        写しパス: "shaders/xpbd_projection.slang",
        写しの前置き: "static const float minimumDistance = ",
    },
    定数の組 {
        正本パス: "crates/blitz_sim/src/xpbd/distance_canonical_projection.rs",
        正本の前置き: "const 向きが定まる最小の距離のメートル: f32 = ",
        写しパス: "shaders/xpbd_bending_projection.slang",
        写しの前置き: "static const float minimumEdgeLength = ",
    },
    定数の組 {
        正本パス: "crates/blitz_sim/src/xpbd/bending_surface_projection.rs",
        正本の前置き: "const 面が定まる最小の面積の平方メートル: f32 = ",
        写しパス: "shaders/xpbd_bending_projection.slang",
        写しの前置き: "static const float minimumAreaVectorLength = ",
    },
];
