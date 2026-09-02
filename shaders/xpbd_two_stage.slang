// 方式C「二段階」: 拘束パスで拘束1本につき1スレッドがΔλと両端の補正の候補を拘束側へ書き、
// 点パスで点1つにつき1スレッドが隣接表を辿って自分に関わる候補を集めて位置へ反映する。
// 書く先が自分の拘束(候補)と自分の点(位置)だけであるため原子演算が要らず、隣接表の順に足すため
// 結果はビット単位で決定的である(判断7)。全拘束を同じ位置から射影するため、原子加算の方式と同じ緩和係数を掛ける。
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
[[vk::binding(6, 0)]] RWStructuredBuffer<float4> correctionCandidates;
[[vk::binding(7, 0)]] StructuredBuffer<uint> adjacencyOffsets;
[[vk::binding(8, 0)]] StructuredBuffer<uint> adjacencyEntries;

static const uint threadsPerGroup = 64;

[shader("compute")]
[numthreads(64, 1, 1)]
void constraintCandidateMain(uint3 id: SV_DispatchThreadID)
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
    float omega = relaxationFactor(degreeOf(adjacencyOffsets, constraint.a), degreeOf(adjacencyOffsets, constraint.b));
    // 退化した拘束はΔλも補正も0であるため、候補へ0を書けば集める側は分岐なしで読める。
    lambdas[index] += projection.deltaLambda * omega;
    correctionCandidates[index * 2] = float4(projection.correctionA * omega, 0.0);
    correctionCandidates[index * 2 + 1] = float4(projection.correctionB * omega, 0.0);
}

// 隣接表の1語は拘束添字の2倍に側(0=a、1=b)を足した値であり、候補の並び(拘束ごとにa・bの順)の添字そのものになる。
[shader("compute")]
[numthreads(64, 1, 1)]
void gatherMain(uint3 id: SV_DispatchThreadID)
{
    uint index = id.x;
    if (index >= params.counts.x)
    {
        return;
    }
    float4 positionInvMass = points[index];
    if (positionInvMass.w == 0.0)
    {
        return;
    }
    float3 correction = float3(0.0, 0.0, 0.0);
    uint end = adjacencyOffsets[index + 1];
    for (uint entry = adjacencyOffsets[index]; entry < end; entry++)
    {
        correction += correctionCandidates[adjacencyEntries[entry]].xyz;
    }
    points[index] = float4(positionInvMass.xyz + correction, positionInvMass.w);
}
