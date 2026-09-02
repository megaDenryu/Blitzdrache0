// Issue #37: 布の目標拘束(判断6)。固定・アタッチ・掴むを、粒子を外から与えた目標位置へ一致させるXPBDの拘束として、
// 距離拘束と同じ反復の中で解く。固定とアタッチだけの別の疑似物理ループ(旧cloth_attach.slangの位置の書き込み)は持たない。
// 目標の確定(targetUpdateMain)は拘束の反復の前に、スキン済み頂点の位置を目標位置バッファへ写す工程である(判断8の工程2。動く目標)。
// 世界固定点と掴みの目標はこの工程が触らず、初期値と介入(cloth_step.slang)がそのまま残る。
// 拘束(targetConstraintMain)は目標拘束1本につき1スレッド。同じ粒子を持つ目標拘束は高々1本(CPUの`目標拘束のバッチ`の生成が保証)であるため
// バッチ全体が彩色の1色であり、距離拘束の色の後ろへ1回のディスパッチで積む。有効が0.0の拘束は読まない(乗数も0のまま)。
// 正典式はxpbd_projection.slangのprojectDistanceを静止長0・目標側の逆質量0で読み、式をここへ再び書かない。
// 乗数は距離拘束の乗数の後ろへ続けて置き、区間の開始(lambdaStart=距離拘束の数)をプッシュ定数で受ける。
// バインディング表と定数レイアウトはcloth_step.slang冒頭の仕様に従う。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import xpbd_projection;

struct ClothParams
{
    float4 gravityDt;
    float4 gridOrigin;
    float4 capsuleA;
    float4 capsuleB;
    uint4 counts;
    float4 misc;
};

// 1本8バイト。`blitz_sim::gpu_layout::xpbd`の目標拘束の引数の契約と同じである。
struct TargetParams
{
    uint particle;
    float compliance;
};

// 目標の確定の対応1件(目標拘束添字・スキン済み頂点添字)。
struct TargetUpdatePair
{
    uint constraintIndex;
    uint skinnedVertex;
};

// 目標拘束の区間。lambdaStartは乗数バッファの中で目標拘束が始まる位置(距離拘束の数)、countは本数である。
// 目標の確定のディスパッチも同じ2語で受け、countだけを対応の件数として読む。
struct TargetRange
{
    uint lambdaStart;
    uint count;
};

[[vk::binding(0, 0)]] ConstantBuffer<ClothParams> params;
[[vk::binding(1, 0)]] RWStructuredBuffer<float4> particles;
[[vk::binding(8, 0)]] StructuredBuffer<float> skinnedVertices;
[[vk::binding(9, 0)]] StructuredBuffer<TargetUpdatePair> targetUpdatePairs;
[[vk::binding(10, 0)]] RWStructuredBuffer<float> lambdas;
[[vk::binding(11, 0)]] StructuredBuffer<TargetParams> targetParams;
[[vk::binding(12, 0)]] RWStructuredBuffer<float4> targets;
[[vk::push_constant]] TargetRange targetRange;

static const uint floatsPerVertex = 12;

// 動く目標の確定: スキン済み頂点の位置をその刻みの目標位置にし、有効にする。
[shader("compute")]
[numthreads(64, 1, 1)]
void targetUpdateMain(uint3 id: SV_DispatchThreadID)
{
    if (id.x >= targetRange.count)
    {
        return;
    }
    TargetUpdatePair pair = targetUpdatePairs[id.x];
    uint base = pair.skinnedVertex * floatsPerVertex;
    float3 target = float3(skinnedVertices[base], skinnedVertices[base + 1], skinnedVertices[base + 2]);
    targets[pair.constraintIndex] = float4(target, 1.0);
}

// 目標拘束を正典式で1回射影し、乗数と粒子の位置を直接書く。粒子は32バイト(位置と逆質量のfloat4 + 速度のfloat4)である。
[shader("compute")]
[numthreads(64, 1, 1)]
void targetConstraintMain(uint3 id: SV_DispatchThreadID)
{
    if (id.x >= targetRange.count)
    {
        return;
    }
    float4 target = targets[id.x];
    if (target.w == 0.0)
    {
        return;
    }
    TargetParams constraint = targetParams[id.x];
    float4 particle = particles[constraint.particle * 2];
    float alphaTilde = constraint.compliance * params.misc.y;
    uint lambdaIndex = targetRange.lambdaStart + id.x;
    DistanceProjection projection = projectDistance(particle.xyz, particle.w, target.xyz, 0.0, 0.0, alphaTilde, lambdas[lambdaIndex]);
    if (!projection.corrected)
    {
        return;
    }
    lambdas[lambdaIndex] += projection.deltaLambda;
    particles[constraint.particle * 2] = float4(particle.xyz + projection.correctionA, particle.w);
}
