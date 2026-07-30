// 多重散乱のベイク済み画像の生成。無限回の散乱を、球面64方向で測った「1回の散乱を経ても残る割合」を公比とする等比級数の和で閉じる。
// 注意: 数式の正本は`crates/blitz_render/src/atmosphere`(multiscatter_lut.rs・multiscatter_march.rs・
// multiscatter_mapping.rs・sphere_directions.rs)にあり、ここはその写しである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_mapping;
import atmosphere_medium;
import atmosphere_march;
import atmosphere_multiscatter_mapping;
import atmosphere_scatter;

[[vk::binding(0, 0)]] ConstantBuffer<AtmosphereMedium> atmosphereMedium;
[[vk::binding(1, 0)]] Sampler2D transmittanceLut;
[[vk::binding(2, 0)]] [format("rgba16f")] RWTexture2D<float4> multiScatterLut;

static const uint threadsPerSide = 8;
// 等比級数の公比が取ってよい上限。正本は`crates/blitz_render/src/atmosphere/integration/multiscatter_series.rs`の`公比の上限`であり、
// ここはその写しである。両者の値が一致することは`cargo xtask conform`が機械検査する。
static const float multiScatterRatioLimit = 0.999;
// 上限を破ったときに書く値。多重散乱は非負であるため、負値は正常な出力に現れない。
// この値が書かれた事実は読み戻し検査の「負」の件数が捉える。
static const float ratioBrokenMarker = -1.0;

[shader("compute")]
[numthreads(threadsPerSide, threadsPerSide, 1)]
void computeMain(uint3 threadId : SV_DispatchThreadID)
{
    uint width;
    uint height;
    multiScatterLut.GetDimensions(width, height);
    if (threadId.x >= width || threadId.y >= height)
    {
        return;
    }

    float2 unitPosition = float2(threadId.xy) / (float2(width, height) - 1.0);
    MultiScatterCondition condition = multiScatterConditionFromUnit(atmosphereMedium, unitPosition);
    float sunZenithCosine = condition.sunZenithCosine;

    // 天頂を+Zに取る。方位角は結果に効かないため、太陽をXZ平面へ置く。
    float3 sunDirection = float3(sqrt(max(0.0, 1.0 - sunZenithCosine * sunZenithCosine)), 0.0, sunZenithCosine);
    float3 observerPosition = float3(0.0, 0.0, condition.radius);

    float3 luminance = float3(0.0, 0.0, 0.0);
    float3 survival = float3(0.0, 0.0, 0.0);
    for (uint index = 0; index < sphereDirectionCount; ++index)
    {
        MarchResult result = marchDirection(
            atmosphereMedium, transmittanceLut, observerPosition, sphereDirection(index), sunDirection);
        luminance += result.luminance;
        survival += result.survival;
    }

    float solidAnglePerDirection = 4.0 * atmospherePi / float(sphereDirectionCount);
    float isotropicPhase = 1.0 / (4.0 * atmospherePi);
    float3 singleScatter = luminance * solidAnglePerDirection * isotropicPhase;
    float3 ratio = survival * solidAnglePerDirection * isotropicPhase;
    // 公比が上限に達するのは媒体の不変条件が破れたときだけである。CPU正本は同じ上限で型付きエラーを返し、
    // 失敗を返せないGPUはここで異常の印を書く。無言で1未満へ抑えると、CPUが失敗させる媒体をGPUだけが
    // それらしい値で描き続けることになり、食い違いが検査から隠れる。NaNは比較が偽になるため別に見る。
    bool ratioBroken = any(ratio >= multiScatterRatioLimit) || any(isnan(ratio));
    if (ratioBroken)
    {
        multiScatterLut[threadId.xy] = float4(ratioBrokenMarker, ratioBrokenMarker, ratioBrokenMarker, 0.0);
        return;
    }
    multiScatterLut[threadId.xy] = float4(singleScatter / (1.0 - ratio), 0.0);
}
