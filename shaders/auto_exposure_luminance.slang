// 自動露出が画素をヒストグラムの枠へ振り分ける判定。相対輝度を求め、共有した線形の境界値との比較でビンを決める。
// 注意: 数式と定数の正本は`crates/blitz_engine/src/auto_exposure/luminance.rs`と`histogram_boundary.rs`にあり、
// ここはその写しである。重みの値の一致は`cargo xtask conform`の定数一致検査が見る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import auto_exposure_scale;
import unfused_arithmetic;

// 相対輝度Y = 0.2126R + 0.7152G + 0.0722B(BT.709の係数)の3成分の重み。
static const float redWeight = 0.2126;
static const float greenWeight = 0.7152;
static const float blueWeight = 0.0722;

// 線形RGBから相対輝度を求める。
// 注意: 3つの積を先に丸め、赤と緑の和を採ってから青を足す、この演算の順そのものがCPU正本との一致契約である。
// 積和融合を禁じるのは、融合すると境界のすぐ近くの画素が共有した境界の反対側のビンへ落ちるためである。
// 線形の境界値を共有しても、輝度を作る演算が違えばビンは一致しない。禁止は`unfused_arithmetic`が生成物の水準で行う。
float relativeLuminance(float3 color)
{
    float weightedRed = unfusedMultiply(color.r, redWeight);
    float weightedGreen = unfusedMultiply(color.g, greenWeight);
    float weightedBlue = unfusedMultiply(color.b, blueWeight);
    return unfusedAdd(unfusedAdd(weightedRed, weightedGreen), weightedBlue);
}

// 輝度が入るビンの添字を、線形の境界値との比較だけで求める。範囲の外は捨てずに端のビンへ張り付ける。
// 区間は下端を含み上端を含まないため、境界ちょうどの輝度は明るい側のビンへ入る。
// 注意: 二分探索の刻み方はCPU正本と同じである。境界の判定に2の対数を使わないのは、CPUとGPUの`log2`の近似が
// 境界の近くで別の側を答えうるためであり、この比較の列には超越関数が1つも現れない。
uint findBinIndex(StructuredBuffer<float> boundaries, float positiveLuminance)
{
    uint low = 0;
    uint high = lastBinIndex;
    while (low < high)
    {
        uint middle = low + (high - low + 1) / 2;
        if (boundaries[middle] <= positiveLuminance)
        {
            low = middle;
        }
        else
        {
            high = middle - 1;
        }
    }
    return low;
}

// 画素が入るヒストグラムのバッファの位置。0以下と非有限はビンでなく専用の枠へ入る。
uint findHistogramSlot(StructuredBuffer<float> boundaries, float3 color)
{
    float luminance = relativeLuminance(color);
    if (isnan(luminance) || isinf(luminance))
    {
        return nonFiniteSlotIndex;
    }
    if (luminance <= 0.0)
    {
        return nonPositiveSlotIndex;
    }
    return findBinIndex(boundaries, luminance);
}
