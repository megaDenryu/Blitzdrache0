// 方式B「グラフ彩色」: 同じ点を同時に触らない拘束の集合(色)ごとに1回のディスパッチを積み、1色の中では拘束1本につき
// 1スレッドが乗数と両端の位置を直接書く。色の区間はプッシュ定数で受ける。
// 色の中で点を共有しないことをCPUの彩色(`crates/blitz_sim/src/constraint_graph/coloring.rs`)が保証するため、
// 原子演算も別の累積域も要らず、結果はビット単位で決定的である(判断7)。
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

// 1色の区間。並べ替えた拘束一覧の中で色が占める連続した範囲であり、CPUの`色の区間`と同じ2語である。
struct ColorRange
{
    uint start;
    uint count;
};

[[vk::binding(0, 0)]] ConstantBuffer<XpbdParams> params;
[[vk::binding(1, 0)]] RWStructuredBuffer<float4> points;
[[vk::binding(3, 0)]] StructuredBuffer<ConstraintParams> constraints;
[[vk::binding(4, 0)]] RWStructuredBuffer<float> lambdas;
[[vk::push_constant]] ColorRange colorRange;

static const uint threadsPerGroup = 64;

[shader("compute")]
[numthreads(64, 1, 1)]
void constraintColoredMain(uint3 id: SV_DispatchThreadID)
{
    if (id.x >= colorRange.count)
    {
        return;
    }
    uint index = colorRange.start + id.x;
    ConstraintParams constraint = constraints[index];
    float4 a = points[constraint.a];
    float4 b = points[constraint.b];
    float alphaTilde = constraint.compliance * params.accelerationDt2.w;
    DistanceProjection projection = projectDistance(a.xyz, a.w, b.xyz, b.w, constraint.restLength, alphaTilde, lambdas[index]);
    if (!projection.corrected)
    {
        return;
    }
    lambdas[index] += projection.deltaLambda;
    points[constraint.a] = float4(a.xyz + projection.correctionA, a.w);
    points[constraint.b] = float4(b.xyz + projection.correctionB, b.w);
}
