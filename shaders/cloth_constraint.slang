// M9: 距離拘束のgather反復(判断52・54)。粒子ごと1スレッドで自分に関わる拘束(最大8本)を読み、
// 自分の位置だけを逆質量比で補正する(原子演算不要)。
// 注意: 同一バッファを他スレッドが同時に補正するため、読む相手位置は同一反復内の新旧が混在する
// (chaotic relaxation)。収束先は同じだがビット単位の決定性は無い。厳密な決定性が要る場合は
// 位置のダブルバッファ化(分類3)で対応する。
// バインディング表と定数レイアウトはcloth_step.slang冒頭の仕様に従う。

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
// 1粒子8スロット。スロット=相手添字u32(空きは0xFFFFFFFF)+静止長f32。
struct NeighborSlot
{
    uint otherIndex;
    float restLength;
};
[[vk::binding(4, 0)]] StructuredBuffer<NeighborSlot> neighborConstraints;

static const uint slotsPerParticle = 8;
static const uint emptySlot = 0xFFFFFFFF;

// XPBD剛距離拘束の自分側補正(blitz_simのCPU参照計算と同式):
// C = |d| - 静止長、補正 = -(w_self / (w_self + w_other)) * C * d/|d| に過緩和係数を掛ける。
[shader("compute")]
[numthreads(64, 1, 1)]
void constraintMain(uint3 id: SV_DispatchThreadID)
{
    uint index = id.x;
    if (index >= params.counts.x)
    {
        return;
    }
    float4 positionInvMass = particles[index * 2];
    float selfInvMass = positionInvMass.w;
    if (selfInvMass == 0.0)
    {
        return;
    }
    float3 position = positionInvMass.xyz;
    float3 correction = float3(0.0, 0.0, 0.0);
    float activeCount = 0.0;
    for (uint slot = 0; slot < slotsPerParticle; slot++)
    {
        NeighborSlot neighbor = neighborConstraints[index * slotsPerParticle + slot];
        if (neighbor.otherIndex == emptySlot)
        {
            continue;
        }
        float4 otherPositionInvMass = particles[neighbor.otherIndex * 2];
        float3 difference = position - otherPositionInvMass.xyz;
        float length2 = dot(difference, difference);
        if (length2 < 1e-12)
        {
            continue;
        }
        float currentLength = sqrt(length2);
        float violation = currentLength - neighbor.restLength;
        float weightSum = selfInvMass + otherPositionInvMass.w;
        correction += difference * (-(selfInvMass / weightSum) * violation / currentLength);
        activeCount += 1.0;
    }
    if (activeCount == 0.0)
    {
        return;
    }
    // 注意: gather合算の補正は関与拘束数で平均する。平均しないと最大8本ぶんの補正が
    // 重なって過剰補正になり、数フレームで発散する(実測で確認した)。
    particles[index * 2].xyz = position + correction * (params.misc.y / activeCount);
}
