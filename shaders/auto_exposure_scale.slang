// 自動露出のヒストグラムの目盛。範囲の下端・ビンの幅・本数と、ビンが代表する対数輝度を持つ。
// 注意: 数式と定数の正本は`crates/blitz_engine/src/auto_exposure/histogram_scale.rs`にあり、ここはその写しである。
// 値の一致は`cargo xtask conform`の定数一致検査が見る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import unfused_arithmetic;

// ヒストグラムが覆う範囲の下端。
static const float rangeLowerLogLuminance = -14.0;
// 1つのビンが覆う対数輝度の幅。
static const float binLogLuminanceWidth = 0.15625;
// ビンの本数。
static const uint binCount = 128;
// 最後のビンの添字。
static const uint lastBinIndex = 127;
// 添字0のビンが代表する対数輝度。範囲の下端にビンの幅の半分を足した値である。
static const float firstBinRepresentativeLogLuminance = -13.921875;

// ビンへ入らない画素を数える2つの枠。ヒストグラムのバッファはビンの後ろにこの2つを並べる。
static const uint nonPositiveSlotIndex = 128;
static const uint nonFiniteSlotIndex = 129;
// ヒストグラムのバッファの要素数。ビンの本数と2つの枠の合計である。
static const uint histogramSlotCount = 130;

// このビンが覆う区間の下端。
// 注意: 積と和の丸めを分けるのは、CPU正本が`範囲の下端 + f32::from(添字) * 幅`を積和融合なしで評価するためである。
// 融合すると代表値が1つ違う単精度になり、切り詰めた平均がCPUと食い違う。
float binLowerLogLuminance(uint index)
{
    return unfusedAdd(rangeLowerLogLuminance, unfusedMultiply(float(index), binLogLuminanceWidth));
}

// このビンを代表する対数輝度。区間の中央を採る。
float binRepresentativeLogLuminance(uint index)
{
    return unfusedAdd(binLowerLogLuminance(index), unfusedMultiply(binLogLuminanceWidth, 0.5));
}

// 添字の平均が代表する対数輝度。ビンの代表値が添字の1次式であるため、代表値の平均は平均の添字の代表値に等しい。
// 128項の浮動小数の総和を採らずに済むのはこの性質による。
float indexMeanLogLuminance(float indexMean)
{
    return unfusedAdd(unfusedMultiply(indexMean, binLogLuminanceWidth), firstBinRepresentativeLogLuminance);
}
