// Issue #38: 布の曲げ拘束(判断11)。隣接する2つの三角形の二面角を静止角へ保つXPBDの拘束を、距離拘束と同じ反復の中で解く工程。
// 並列方式はグラフ彩色(判断7の採用規則1): 4粒子を触るため距離拘束とは別の彩色であり、色の区間ごとに1回のディスパッチを積む。
// 1色の中では拘束1本につき1スレッドが乗数と4粒子の位置を直接書く。色の中で粒子を共有しないことをCPUの彩色(`blitz_sim::曲げ拘束の彩色`)が保証する。
// 各反復では距離拘束の色の後ろ・目標拘束の前に積む(判断8)。後ろに置くのは目標拘束であり、曲げを目標拘束の後ろに置くと固定した粒子を曲げが動かして固定が緩む。
// 正典式は`xpbd_bending_projection.slang`の1つの写しを読み、式をここへ再び書かない。退化(全て固定・辺や三角形が潰れている)は補正0で乗数を変えない。
// 乗数は距離拘束と目標拘束の乗数の後ろへ続けて置き、区間の開始(lambdaStart=距離拘束の数+目標拘束の数)をプッシュ定数の3語目で受ける。
// バインディング表と定数レイアウトはcloth_step.slang冒頭の仕様に従う(b13=曲げ拘束の引数)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import xpbd_bending_projection;

struct ClothParams
{
    float4 gravityDt;
    float4 gridOrigin;
    float4 capsuleA;
    float4 capsuleB;
    uint4 counts;
    float4 misc;
};

// 1拘束24バイト。並びは彩色で色ごとに並べ替えた順であり、`blitz_sim::gpu_layout::xpbd`の曲げ拘束の引数の契約と同じである。
struct BendingParams
{
    uint a;
    uint b;
    uint c;
    uint d;
    float restAngle;
    float compliance;
};

// 1色の区間と乗数の区間の開始。startとcountは並べ替えた曲げ拘束一覧の中で色が占める連続した範囲、lambdaStartは乗数バッファの中で曲げ拘束が始まる位置である。
struct BendingRange
{
    uint start;
    uint count;
    uint lambdaStart;
};

[[vk::binding(0, 0)]] ConstantBuffer<ClothParams> params;
[[vk::binding(1, 0)]] RWStructuredBuffer<float4> particles;
[[vk::binding(10, 0)]] RWStructuredBuffer<float> lambdas;
[[vk::binding(13, 0)]] StructuredBuffer<BendingParams> bendingConstraints;
[[vk::push_constant]] BendingRange bendingRange;

static const uint threadsPerGroup = 64;

// 1色の区間の曲げ拘束を正典式で1回射影し、乗数と4粒子の位置を直接書く。粒子は32バイト(位置と逆質量のfloat4 + 速度のfloat4)である。
[shader("compute")]
[numthreads(64, 1, 1)]
void bendingConstraintMain(uint3 id: SV_DispatchThreadID)
{
    if (id.x >= bendingRange.count)
    {
        return;
    }
    uint index = bendingRange.start + id.x;
    BendingParams constraint = bendingConstraints[index];
    float4 a = particles[constraint.a * 2];
    float4 b = particles[constraint.b * 2];
    float4 c = particles[constraint.c * 2];
    float4 d = particles[constraint.d * 2];
    float alphaTilde = constraint.compliance * params.misc.y;
    uint lambdaIndex = bendingRange.lambdaStart + index;
    BendingProjection projection = projectDihedralBending(
        a.xyz, a.w, b.xyz, b.w, c.xyz, c.w, d.xyz, d.w, constraint.restAngle, alphaTilde, lambdas[lambdaIndex]);
    if (!projection.corrected)
    {
        return;
    }
    lambdas[lambdaIndex] += projection.deltaLambda;
    particles[constraint.a * 2] = float4(a.xyz + projection.correctionA, a.w);
    particles[constraint.b * 2] = float4(b.xyz + projection.correctionB, b.w);
    particles[constraint.c * 2] = float4(c.xyz + projection.correctionC, c.w);
    particles[constraint.d * 2] = float4(d.xyz + projection.correctionD, d.w);
}
