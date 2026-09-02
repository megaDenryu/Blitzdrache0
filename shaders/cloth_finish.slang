// M9: 布の衝突解決と速度更新(判断52・54)。床(y=0)とキャラクター近似カプセル1本からの
// 押し出しの後、XPBDの速度更新 v=(位置-前位置)/dt に減衰を掛ける。固定した粒子も目標拘束で縛られているだけであり、区別せず同じ規則を当てる。
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

[shader("compute")]
[numthreads(64, 1, 1)]
void finishMain(uint3 id: SV_DispatchThreadID)
{
    uint index = id.x;
    if (index >= params.counts.x)
    {
        return;
    }
    float4 positionInvMass = particles[index * 2];
    float3 position = pushOutOfCapsule(positionInvMass.xyz);
    if (position.y < 0.0)
    {
        position.y = 0.0;
    }
    float dt = params.gravityDt.w;
    float3 velocity = (position - previousPositions[index].xyz) / dt * params.misc.z;
    particles[index * 2].xyz = position;
    particles[index * 2 + 1].xyz = velocity;
}
