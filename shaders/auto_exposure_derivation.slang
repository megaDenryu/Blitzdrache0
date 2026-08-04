// ヒストグラムから目標の露出補正段を導く工程。上下1パーセントを除いた窓での対数輝度の平均から目標を作る。
// 注意: 定義の正本は`crates/blitz_engine/src/auto_exposure/derivation.rs`にあり、ここはその写しである。
// 総和を添字の昇順で採ることは正本の定義の一部であり、並列の浮動小数リダクションに置き換えてはならない
// (浮動小数の加算は順序で結果が変わる)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import auto_exposure_scale;
import unfused_arithmetic;

// 片側で除外する件数を決める除数。上下それぞれ1パーセントを外すため、総数を100で割る。
static const uint oneSideExclusionDivisor = 100;

// 導出の結果。導出できないフレームでは`resolved`が偽になり、呼び出し側は状態を1つも変えない。
struct DerivationResult
{
    bool resolved;
    float targetCompensationStep;
};

// 添字の昇順にビンを足した総数。
uint totalBinnedPixelCount(StructuredBuffer<uint> histogram)
{
    uint total = 0;
    for (uint index = 0; index < binCount; ++index)
    {
        total += histogram[index];
    }
    return total;
}

// 上下1パーセントを除いた窓の中で、ビンが代表する対数輝度を件数で重み付けた平均。
// 窓の端はビンの途中に落ちうるため、そのビンからは窓に重なる件数だけを取り出す。
float truncatedMeanLogLuminance(StructuredBuffer<uint> histogram, uint total, uint windowCount)
{
    uint windowBegin = total / oneSideExclusionDivisor;
    uint windowEnd = total - windowBegin;
    uint scanned = 0;
    float weightedSum = 0.0;
    for (uint index = 0; index < binCount; ++index)
    {
        uint binBegin = scanned;
        uint binEnd = scanned + histogram[index];
        scanned = binEnd;
        uint upper = min(binEnd, windowEnd);
        uint lower = max(binBegin, windowBegin);
        uint overlap = upper > lower ? upper - lower : 0;
        float term = unfusedMultiply(float(overlap), binRepresentativeLogLuminance(index));
        weightedSum = unfusedAdd(weightedSum, term);
    }
    return weightedSum / float(windowCount);
}

// 目標の露出補正段を導く。中間灰へ写すための段は「目標中間灰の対数輝度 - 切り詰めた平均」である。
DerivationResult deriveTargetCompensationStep(
    StructuredBuffer<uint> histogram, float middleGrayLogLuminance, float lowerLimit, float upperLimit)
{
    DerivationResult result;
    result.resolved = false;
    result.targetCompensationStep = 0.0;

    uint total = totalBinnedPixelCount(histogram);
    if (total == 0)
    {
        return result;
    }
    uint oneSideExclusion = total / oneSideExclusionDivisor;
    uint windowCount = total - oneSideExclusion * 2;
    if (windowCount == 0)
    {
        return result;
    }
    float mean = truncatedMeanLogLuminance(histogram, total, windowCount);
    result.resolved = true;
    result.targetCompensationStep = clamp(unfusedSubtract(middleGrayLogLuminance, mean), lowerLimit, upperLimit);
    return result;
}
