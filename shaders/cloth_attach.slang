// M9: アタッチ拘束(判断52)。布の指定粒子をスキン済み頂点バッファの指定頂点位置へ固定する
// (判断44の介入合流点=スキン済みバッファを後段コンピュートが読む、の実証)。
// 対応表の件数はcounts.wで渡す。バインディング表はcloth_step.slang冒頭の仕様に従う
// (b8=スキン済み頂点(48バイト=float12個/頂点)、b9=アタッチ対応(布粒子添字u32+スキン頂点添字u32))。

struct ClothParams
{
    float4 gravityDt;
    float4 gridOrigin;
    float4 capsuleA;
    float4 capsuleB;
    uint4 counts;
    float4 misc;
};

[[vk::binding(0, 0)]] ConstantBuffer<ClothParams> params;
[[vk::binding(1, 0)]] RWStructuredBuffer<float4> particles;
[[vk::binding(2, 0)]] RWStructuredBuffer<float4> previousPositions;
[[vk::binding(8, 0)]] StructuredBuffer<float> skinnedVertices;
struct AttachPair
{
    uint clothParticle;
    uint skinnedVertex;
};
[[vk::binding(9, 0)]] StructuredBuffer<AttachPair> attachPairs;

static const uint floatsPerVertex = 12;

// 固定は「毎フレーム位置を目標へ移し、逆質量0を維持する」ことで表す。速度は0にする
// (アタッチ先の移動速度を布へ伝えるのは前位置との差分=速度更新パスが自然に行う)。
[shader("compute")]
[numthreads(64, 1, 1)]
void attachMain(uint3 id: SV_DispatchThreadID)
{
    uint index = id.x;
    if (index >= params.counts.w)
    {
        return;
    }
    AttachPair pair = attachPairs[index];
    uint base = pair.skinnedVertex * floatsPerVertex;
    float3 target = float3(skinnedVertices[base], skinnedVertices[base + 1], skinnedVertices[base + 2]);
    particles[pair.clothParticle * 2] = float4(target, 0.0);
    particles[pair.clothParticle * 2 + 1] = float4(0.0, 0.0, 0.0, 0.0);
}
