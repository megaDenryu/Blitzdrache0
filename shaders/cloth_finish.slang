// M9: 布の衝突解決(床とカプセルの押し出し)と速度の再構成(判断52・54)。2つの入口を持つ。
// collisionPushOutMainは床(y=0)とキャラクター近似カプセル1本の内側にある粒子を表面へ押し出し、位置だけを書く。
// finishMainはXPBDの速度更新 v=(位置-前位置)/dt に減衰を掛け、速度だけを書く。
// 2つの入口に分けるのは、押し出しと速度の再構成の間に目標拘束の最終の成立(cloth_target.slangのtargetConstraintMain)が入るためである
// (`_doc/設計/XPBD共通拘束基盤.md`「判断6」)。分離と押し出しは全粒子を動かすため、その後に目標拘束を成立させてから速度を作らないと、
// 固定・アタッチ・掴んだ粒子が目標から外れた位置に残り、次の刻みへ誤った速度を持ち越す。
// 固定した粒子も目標拘束で縛られているだけであり、区別せず同じ規則を当てる。
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
[[vk::binding(2, 0)]] RWStructuredBuffer<float4> previousPositions;

// カプセル(線分AB+半径)表面より内側の点を最近傍方向へ押し出す。
float3 pushOutOfCapsule(float3 position)
{
    float3 segment = params.capsuleB.xyz - params.capsuleA.xyz;
    float segmentLength2 = dot(segment, segment);
    float t = segmentLength2 < 1e-12 ? 0.0 : clamp(dot(position - params.capsuleA.xyz, segment) / segmentLength2, 0.0, 1.0);
    float3 closest = params.capsuleA.xyz + segment * t;
    float3 offset = position - closest;
    float distance2 = dot(offset, offset);
    float radius = params.capsuleA.w;
    if (distance2 >= radius * radius || distance2 < 1e-12)
    {
        return position;
    }
    float distance = sqrt(distance2);
    return closest + offset * (radius / distance);
}

// 床とカプセルの押し出し。位置だけを書き、速度は触らない。
[shader("compute")]
[numthreads(64, 1, 1)]
void collisionPushOutMain(uint3 id: SV_DispatchThreadID)
{
    uint index = id.x;
    if (index >= params.counts.x)
    {
        return;
    }
    float3 position = pushOutOfCapsule(particles[index * 2].xyz);
    if (position.y < 0.0)
    {
        position.y = 0.0;
    }
    particles[index * 2].xyz = position;
}

// 速度の再構成。確定した位置と刻みの開始の位置の差から速度を作り直す(判断8の工程8)。位置は触らない。
[shader("compute")]
[numthreads(64, 1, 1)]
void finishMain(uint3 id: SV_DispatchThreadID)
{
    uint index = id.x;
    if (index >= params.counts.x)
    {
        return;
    }
    float3 position = particles[index * 2].xyz;
    float dt = params.gravityDt.w;
    particles[index * 2 + 1].xyz = (position - previousPositions[index].xyz) / dt * params.misc.z;
}
