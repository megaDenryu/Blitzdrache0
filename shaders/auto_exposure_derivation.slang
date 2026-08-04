// ヒストグラムから目標の露出補正段を導く工程。上下1パーセントを除いた窓での対数輝度の平均から目標を作る。
// 注意: 定義の正本は`crates/blitz_engine/src/auto_exposure/derivation.rs`にあり、ここはその写しである。
// 切り詰めた平均は、窓に重なる件数を重みにした添字の整数の総和から作る。整数の加算は足す順にも結合の順にも
// 依存しないため、この中間値はCPU正本と必ずバイト一致する。浮動小数が現れるのは、この整数から平均を作る境界より後だけである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import auto_exposure_index_sum;
import auto_exposure_scale;
import unfused_arithmetic;

// 片側で除外する件数を決める除数。上下それぞれ1パーセントを外すため、総数を100で割る。
static const uint oneSideExclusionDivisor = 100;

// 導出の結果。導出できないフレームでは`resolved`が偽になり、呼び出し側は状態を1つも変えない。
// 整数の中間値も返すのは、検収がCPU正本とバイト一致を見る対象がこの値だからである。
struct DerivationResult
{
    bool resolved;
    float targetCompensationStep;
    WeightedIndexSum weightedIndexSum;
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

// 上下1パーセントを除いた窓の中で、窓に重なる件数を重みにした添字の総和。
// 窓の端はビンの途中に落ちうるため、そのビンからは窓に重なる件数だけを取り出す。
// 走査を1つの呼び出しに保つのは順序のためではなく、窓との重なりを求めるのに走査済みの件数を順に積む必要があるためである。
WeightedIndexSum windowWeightedIndexSum(StructuredBuffer<uint> histogram, uint total)
{
    uint windowBegin = total / oneSideExclusionDivisor;
    uint windowEnd = total - windowBegin;
    uint scanned = 0;
    WeightedIndexSum sum = zeroWeightedIndexSum();
    for (uint index = 0; index < binCount; ++index)
    {
        uint binBegin = scanned;
        uint binEnd = scanned + histogram[index];
        scanned = binEnd;
        uint upper = min(binEnd, windowEnd);
        uint lower = max(binBegin, windowBegin);
        sum = addTerm(sum, upper > lower ? upper - lower : 0, index);
    }
    return sum;
}

// 添字の総和を件数で割って添字の平均を作り、その位置が代表する対数輝度へ写す。
float truncatedMeanLogLuminance(WeightedIndexSum sum, uint windowCount)
{
    return indexMeanLogLuminance(weightedIndexSumAsFloat(sum) / float(windowCount));
}

// 目標の露出補正段を導く。中間灰へ写すための段は「目標中間灰の対数輝度 - 切り詰めた平均」である。
DerivationResult deriveTargetCompensationStep(
    StructuredBuffer<uint> histogram, float middleGrayLogLuminance, float lowerLimit, float upperLimit)
{
    DerivationResult result;
    result.resolved = false;
    result.targetCompensationStep = 0.0;
    result.weightedIndexSum = zeroWeightedIndexSum();

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
    result.weightedIndexSum = windowWeightedIndexSum(histogram, total);
    float mean = truncatedMeanLogLuminance(result.weightedIndexSum, windowCount);
    result.resolved = true;
    result.targetCompensationStep = clamp(unfusedSubtract(middleGrayLogLuminance, mean), lowerLimit, upperLimit);
    return result;
}
