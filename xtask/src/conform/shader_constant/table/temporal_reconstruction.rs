//! 時間再構成の正本と、その写しの対応。突き合わせるのは相対輝度の3つの重みだけである。
//!
//! 3つを2組で見るのは、写しが2箇所にあるからである。1つはslangの写しであり、GPUの混合はCPU正本と同じ重みで
//! 明るい側の寄与を下げなければならない。値がずれると残像の抑え方が正本と食い違うが、絵はそれらしく出てしまう。
//! もう1つは自動露出のCPU正本であり、こちらは写しでなく2つ目の正本である。同じ線形の色に対して2つの輝度の定義が
//! 並ぶと、露出が明るいと判定した画素を混合が暗いと重み付ける食い違いが起こる。
//!
//! 列の長さ・ハルトン列の基数・今のフレームの寄与率・出現領域の相対許容はここに現れない。画素内ずらしはCPUだけで
//! 回してビュー射影へ畳み込むため列の写しが要らず、混合の2つの数は即時定数としてGPUへ運ぶため値の写しが要らない。
//! 写しの無い定数を台帳へ載せると、突き合わせのためだけの2つ目の宣言を作ることになる。

use super::定数の組;

const 正本: &str = "crates/blitz_engine/src/temporal_reconstruction/constants.rs";
const シェーダーの写し: &str = "shaders/temporal_reconstruction_blend.slang";
const 自動露出の正本: &str = "crates/blitz_engine/src/auto_exposure/luminance.rs";

pub(super) const 定数一覧: [定数の組; 6] = [
    定数の組 {
        正本パス: 正本,
        正本の前置き: "pub(super) const 赤の重み: f32 = ",
        写しパス: シェーダーの写し,
        写しの前置き: "static const float temporalRedWeight = ",
    },
    定数の組 {
        正本パス: 正本,
        正本の前置き: "pub(super) const 緑の重み: f32 = ",
        写しパス: シェーダーの写し,
        写しの前置き: "static const float temporalGreenWeight = ",
    },
    定数の組 {
        正本パス: 正本,
        正本の前置き: "pub(super) const 青の重み: f32 = ",
        写しパス: シェーダーの写し,
        写しの前置き: "static const float temporalBlueWeight = ",
    },
    定数の組 {
        正本パス: 正本,
        正本の前置き: "pub(super) const 赤の重み: f32 = ",
        写しパス: 自動露出の正本,
        写しの前置き: "pub(super) const 赤の重み: f32 = ",
    },
    定数の組 {
        正本パス: 正本,
        正本の前置き: "pub(super) const 緑の重み: f32 = ",
        写しパス: 自動露出の正本,
        写しの前置き: "pub(super) const 緑の重み: f32 = ",
    },
    定数の組 {
        正本パス: 正本,
        正本の前置き: "pub(super) const 青の重み: f32 = ",
        写しパス: 自動露出の正本,
        写しの前置き: "pub(super) const 青の重み: f32 = ",
    },
];
