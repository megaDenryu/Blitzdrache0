//! 自動露出の正本と、その写しの対応。写しは2箇所にある。
//!
//! 1つはxtask自身の中にある。範囲の制定に使う実測の入口(`cargo xtask hdr-luminance`)が外部依存を持たない方針を保ち、
//! blitz_engineを読めないためである。重みが片方だけ動くと、制定したヒストグラムの範囲が実測の分布と対応しなくなる。
//!
//! もう1つはslangの写しである。GPUの集計と導出はCPU正本と同じ重み・同じ目盛・同じ除数で動かなければならず、
//! 値がずれるとビンが1つずれた分布や別の窓での平均になる。絵はそれらしく出てしまうため、ここが機械的に見る。
//! 境界の線形輝度の列だけは定数の写しを持たない(コンポジションルートがCPU正本の列をGPUのバッファへ運ぶ)。
//!
//! ビンの本数はblitz_renderのGPU境界の宣言にも写しがあり、その2つはクレート同士の対である。
//! slang側でビンの本数を直に持つ宣言は0以下の枠の添字であり、ビンが添字0から本数-1までを占める以上、
//! 枠の添字と本数は同じ値でなければならない。ヒストグラムのバッファの要素数(本数+2)は
//! CPU側が式で書くためこの検査では読めず、本数の一致がその代わりを果たす。

use super::定数の組;

const 輝度の正本: &str = "crates/blitz_engine/src/auto_exposure/luminance.rs";
const 目盛の正本: &str = "crates/blitz_engine/src/auto_exposure/histogram_scale.rs";
const ビン数の前置き: &str = "pub const ビン数: usize = ";
const 実測の入口の写し: &str = "xtask/src/relative_luminance.rs";
const 輝度の写し: &str = "shaders/auto_exposure_luminance.slang";
const 目盛の写し: &str = "shaders/auto_exposure_scale.slang";

pub(super) const 定数一覧: [定数の組; 12] = [
    定数の組 {
        正本パス: 目盛の正本,
        正本の前置き: ビン数の前置き,
        写しパス: "crates/blitz_render/src/auto_exposure.rs",
        写しの前置き: ビン数の前置き,
    },
    定数の組 {
        正本パス: 目盛の正本,
        正本の前置き: ビン数の前置き,
        写しパス: 目盛の写し,
        写しの前置き: "static const uint nonPositiveSlotIndex = ",
    },
    定数の組 {
        正本パス: 輝度の正本,
        正本の前置き: "pub(super) const 赤の重み: f32 = ",
        写しパス: 実測の入口の写し,
        写しの前置き: "const 赤の重み: f64 = ",
    },
    定数の組 {
        正本パス: 輝度の正本,
        正本の前置き: "pub(super) const 緑の重み: f32 = ",
        写しパス: 実測の入口の写し,
        写しの前置き: "const 緑の重み: f64 = ",
    },
    定数の組 {
        正本パス: 輝度の正本,
        正本の前置き: "pub(super) const 青の重み: f32 = ",
        写しパス: 実測の入口の写し,
        写しの前置き: "const 青の重み: f64 = ",
    },
    定数の組 {
        正本パス: 輝度の正本,
        正本の前置き: "pub(super) const 赤の重み: f32 = ",
        写しパス: 輝度の写し,
        写しの前置き: "static const float redWeight = ",
    },
    定数の組 {
        正本パス: 輝度の正本,
        正本の前置き: "pub(super) const 緑の重み: f32 = ",
        写しパス: 輝度の写し,
        写しの前置き: "static const float greenWeight = ",
    },
    定数の組 {
        正本パス: 輝度の正本,
        正本の前置き: "pub(super) const 青の重み: f32 = ",
        写しパス: 輝度の写し,
        写しの前置き: "static const float blueWeight = ",
    },
    定数の組 {
        正本パス: 目盛の正本,
        正本の前置き: "pub(super) const 範囲の下端の対数輝度: f32 = ",
        写しパス: 目盛の写し,
        写しの前置き: "static const float rangeLowerLogLuminance = ",
    },
    定数の組 {
        正本パス: 目盛の正本,
        正本の前置き: "pub(super) const ビンが覆う対数輝度の幅: f32 = ",
        写しパス: 目盛の写し,
        写しの前置き: "static const float binLogLuminanceWidth = ",
    },
    定数の組 {
        正本パス: 目盛の正本,
        正本の前置き: "pub const 最後のビンの添字: u8 = ",
        写しパス: 目盛の写し,
        写しの前置き: "static const uint lastBinIndex = ",
    },
    定数の組 {
        正本パス: "crates/blitz_engine/src/auto_exposure/derivation.rs",
        正本の前置き: "const 片側の除外を決める除数: u32 = ",
        写しパス: "shaders/auto_exposure_derivation.slang",
        写しの前置き: "static const uint oneSideExclusionDivisor = ",
    },
];
