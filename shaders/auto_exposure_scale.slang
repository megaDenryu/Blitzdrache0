// 自動露出のヒストグラムの目盛。範囲の下端・ビンの幅・本数と、ビンが代表する対数輝度を持つ。
// 注意: 数式と定数の正本は`crates/blitz_engine/src/auto_exposure/histogram_scale.rs`にあり、ここはその写しである。
// 値の一致は`cargo xtask conform`の定数一致検査が見る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// ヒストグラムが覆う範囲の下端。
static const float rangeLowerLogLuminance = -14.0;
// 1つのビンが覆う対数輝度の幅。
static const float binLogLuminanceWidth = 0.15625;
// ビンの本数。
static const uint binCount = 128;
// 最後のビンの添字。
static const uint lastBinIndex = 127;

// ビンへ入らない画素を数える2つの枠。ヒストグラムのバッファはビンの後ろにこの2つを並べる。
static const uint nonPositiveSlotIndex = 128;
static const uint nonFiniteSlotIndex = 129;
// ヒストグラムのバッファの要素数。ビンの本数と2つの枠の合計である。
static const uint histogramSlotCount = 130;

// このビンが覆う区間の下端。
// 注意: `precise`で積と和の丸めを分けるのは、CPU正本が`範囲の下端 + f32::from(添字) * 幅`を
// 積和融合なしで評価するためである。融合すると代表値が1つ違う単精度になり、切り詰めた平均がCPUと食い違う。
float binLowerLogLuminance(uint index)
{
    precise float scaled = float(index) * binLogLuminanceWidth;
    precise float lower = rangeLowerLogLuminance + scaled;
    return lower;
}

// このビンを代表する対数輝度。区間の中央を採る。
float binRepresentativeLogLuminance(uint index)
{
    precise float halfWidth = binLogLuminanceWidth * 0.5;
    precise float representative = binLowerLogLuminance(index) + halfWidth;
    return representative;
}
