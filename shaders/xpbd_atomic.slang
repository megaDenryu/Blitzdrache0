// 方式A「拘束ごとに1スレッドと原子加算」: 拘束1本につき1スレッドがΔλと両端の補正を計算し、乗数を自分の拘束へ書き、
// 補正を点側の累積へ整数の原子加算で足す。反復ごとに累積を位置へ足す工程を別に積む。
// 浮動小数の原子加算は拡張(shader_atomic_float)に依るため使わず、固定小数へ量子化した32ビット整数の原子加算にする。
// 量子化の桁は定数UBOの尺度が決める(CPU正本 `crates/blitz_render/src/vulkan/xpbd_bench/params.rs`)。
// 注意: 原子加算の順はスレッドの順で変わるため、この方式はビット単位の数値決定性を持たない(判断7)。
// バインディング表とレイアウトはxpbd_step.slang冒頭の仕様に従う。

import xpbd_projection;

struct XpbdParams
{
    float4 accelerationDt2;
    uint4 counts;
    float4 misc;
};

struct ConstraintParams
{
    uint a;
    uint b;
    float restLength;
    float compliance;
};

[[vk::binding(0, 0)]] ConstantBuffer<XpbdParams> params;
[[vk::binding(1, 0)]] RWStructuredBuffer<float4> points;
[[vk::binding(3, 0)]] StructuredBuffer<ConstraintParams> constraints;
[[vk::binding(4, 0)]] RWStructuredBuffer<float> lambdas;
[[vk::binding(5, 0)]] RWStructuredBuffer<int> correctionAccumulators;
[[vk::binding(7, 0)]] StructuredBuffer<uint> adjacencyOffsets;

static const uint threadsPerGroup = 64;

void accumulate(uint pointIndex, float3 correction)
{
    float scale = params.misc.x;
    InterlockedAdd(correctionAccumulators[pointIndex * 3], int(round(correction.x * scale)));
    InterlockedAdd(correctionAccumulators[pointIndex * 3 + 1], int(round(correction.y * scale)));
    InterlockedAdd(correctionAccumulators[pointIndex * 3 + 2], int(round(correction.z * scale)));
}

[shader("compute")]
[numthreads(64, 1, 1)]
void constraintAtomicMain(uint3 id: SV_DispatchThreadID)
{
    uint index = id.x;
    if (index >= params.counts.y)
    {
        return;
    }
    ConstraintParams constraint = constraints[index];
    float4 a = points[constraint.a];
    float4 b = points[constraint.b];
    float alphaTilde = constraint.compliance * params.accelerationDt2.w;
    DistanceProjection projection = projectDistance(a.xyz, a.w, b.xyz, b.w, constraint.restLength, alphaTilde, lambdas[index]);
    if (!projection.corrected)
    {
        return;
    }
    float omega = relaxationFactor(degreeOf(adjacencyOffsets, constraint.a), degreeOf(adjacencyOffsets, constraint.b));
    lambdas[index] += projection.deltaLambda * omega;
    accumulate(constraint.a, projection.correctionA * omega);
    accumulate(constraint.b, projection.correctionB * omega);
}

// 累積した補正を位置へ足し、累積を0へ戻す。点1つにつき1スレッドであり、自分の累積だけを触る。
[shader("compute")]
[numthreads(64, 1, 1)]
void applyAtomicMain(uint3 id: SV_DispatchThreadID)
{
    uint index = id.x;
    if (index >= params.counts.x)
    {
        return;
    }
    float inverseScale = 1.0 / params.misc.x;
    float3 correction = float3(
        float(correctionAccumulators[index * 3]),
        float(correctionAccumulators[index * 3 + 1]),
        float(correctionAccumulators[index * 3 + 2]));
    correctionAccumulators[index * 3] = 0;
    correctionAccumulators[index * 3 + 1] = 0;
    correctionAccumulators[index * 3 + 2] = 0;
    points[index].xyz += correction * inverseScale;
}
