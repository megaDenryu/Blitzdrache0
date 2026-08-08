// 1画素ぶんの再構成の合成そのもの。受け取るのは今のフレームの色・履歴の色・近傍の色の範囲・履歴の採否・
// 今のフレームの寄与率、返すのは再構成後の色である。
// 注意: 合成の順の正本は`crates/blitz_engine/src/temporal_reconstruction/pixel_reconstruction.rs`であり、
// ここはその写しである。順は採否 → 近傍の範囲で収める → 輝度で重み付けて混ぜるの3段であり、入れ替えられない。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import unfused_arithmetic;

// 相対輝度Y = 0.2126R + 0.7152G + 0.0722B(BT.709の係数)の3成分の重み。
// 注意: 値の正本は`crates/blitz_engine/src/temporal_reconstruction/constants.rs`であり、その正本は
// 自動露出の層と同じ値でなければならない。3つの一致は`cargo xtask conform`の定数一致検査が見る。
static const float temporalRedWeight = 0.2126;
static const float temporalGreenWeight = 0.7152;
static const float temporalBlueWeight = 0.0722;

// 線形の色から相対輝度を求める。
// 注意: 3つの積を先に丸め、赤と緑の和を採ってから青を足す、この演算の順そのものがCPU正本との一致契約である。
// 積和融合を禁じるのは、融合すると混合の重みが正本と最下位の桁で食い違い、合成した入力の注入による
// 突き合わせの許容を誤差が食い潰すためである。禁止は`unfused_arithmetic`が生成物の水準で行う。
float temporalRelativeLuminance(float3 color)
{
    float weightedRed = unfusedMultiply(color.r, temporalRedWeight);
    float weightedGreen = unfusedMultiply(color.g, temporalGreenWeight);
    float weightedBlue = unfusedMultiply(color.b, temporalBlueWeight);
    return unfusedAdd(unfusedAdd(weightedRed, weightedGreen), weightedBlue);
}

// 成分1つぶんの重み付き平均。
float temporalMixComponent(float history, float current, float historyWeight, float currentWeight, float weightSum)
{
    float weighted = unfusedAdd(unfusedMultiply(history, historyWeight), unfusedMultiply(current, currentWeight));
    return weighted / weightSum;
}

// 明るい側の寄与を下げてから履歴と今のフレームを混ぜる。両者の輝度が等しいとき、この混合は指数移動平均と一致する。
// 注意: 重みを作る演算の順もCPU正本(`luminance_weighted_blend.rs`)の写しである。
// 割り算は融合の対象にならないため、積と和だけを融合しない演算へ置き換える。
float3 temporalLuminanceWeightedBlend(float3 history, float3 current, float currentFrameRatio)
{
    float historyWeight = unfusedSubtract(1.0, currentFrameRatio) / unfusedAdd(1.0, temporalRelativeLuminance(history));
    float currentWeight = currentFrameRatio / unfusedAdd(1.0, temporalRelativeLuminance(current));
    float weightSum = unfusedAdd(historyWeight, currentWeight);
    return float3(
        temporalMixComponent(history.r, current.r, historyWeight, currentWeight, weightSum),
        temporalMixComponent(history.g, current.g, historyWeight, currentWeight, weightSum),
        temporalMixComponent(history.b, current.b, historyWeight, currentWeight, weightSum));
}

// 1画素ぶんの再構成。採否を最初に見るのは、捨てる画素で履歴の色も近傍の範囲も結果へ1つも関与させないためである。
// 混ぜてから捨てる形にすると、捨てた画素の結果が混合の丸め誤差の分だけ今のフレームの色から離れ、
// 履歴を無効化した直後の1フレーム目が今のフレームと一致するという判断cの規律が画素で破れる。
float3 temporalReconstructPixel(
    float3 currentColor,
    float3 historyColor,
    float3 neighborhoodMinimum,
    float3 neighborhoodMaximum,
    bool admitted,
    float currentFrameRatio)
{
    if (!admitted)
    {
        return currentColor;
    }
    float3 clampedHistory = clamp(historyColor, neighborhoodMinimum, neighborhoodMaximum);
    return temporalLuminanceWeightedBlend(clampedHistory, currentColor, currentFrameRatio);
}
