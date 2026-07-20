// M9: セルフコリジョンの分離(判断52・54)。自セルと隣接26セルの粒子をgatherし、
// 衝突半径の2倍より近い粒子から自分だけを押し出す(原子演算不要。近似の非対称性は
// 相手側スレッドが対称に自分を押すことで解消される)。
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
[[vk::binding(5, 0)]] RWStructuredBuffer<uint> cellCounts;
[[vk::binding(6, 0)]] RWStructuredBuffer<uint> cellSlots;

static const uint gridSide = 32;
static const uint cellCapacity = 8;

[shader("compute")]
[numthreads(64, 1, 1)]
void separateMain(uint3 id: SV_DispatchThreadID)
{
    uint index = id.x;
    if (index >= params.counts.x)
    {
        return;
    }
    float4 positionInvMass = particles[index * 2];
    if (positionInvMass.w == 0.0)
    {
        return;
    }
    float3 position = positionInvMass.xyz;
    float minDistance = params.misc.x * 2.0;
    float3 relative = (position - params.gridOrigin.xyz) / params.gridOrigin.w;
    int3 centerCell = int3(floor(relative));
    float3 correction = float3(0.0, 0.0, 0.0);
    float activeCount = 0.0;
    for (int dz = -1; dz <= 1; dz++)
    {
        for (int dy = -1; dy <= 1; dy++)
        {
            for (int dx = -1; dx <= 1; dx++)
            {
                int3 cell = centerCell + int3(dx, dy, dz);
                if (any(cell < int3(0, 0, 0)) || any(cell >= int3(gridSide, gridSide, gridSide)))
                {
                    continue;
                }
                uint cellIndex = (uint(cell.z) * gridSide + uint(cell.y)) * gridSide + uint(cell.x);
                uint count = min(cellCounts[cellIndex], cellCapacity);
                for (uint slot = 0; slot < count; slot++)
                {
                    uint other = cellSlots[cellIndex * cellCapacity + slot];
                    if (other == index)
                    {
                        continue;
                    }
                    float3 difference = position - particles[other * 2].xyz;
                    float distance2 = dot(difference, difference);
                    if (distance2 < 1e-12 || distance2 >= minDistance * minDistance)
                    {
                        continue;
                    }
                    float distance = sqrt(distance2);
                    correction += difference * ((minDistance - distance) * 0.5 / distance);
                    activeCount += 1.0;
                }
            }
        }
    }
    if (activeCount == 0.0)
    {
        return;
    }
    // 注意: 拘束gatherと同じ理由で、複数近傍からの押し出し合算は近傍数で平均する(過剰補正で発散する)。
    particles[index * 2].xyz = position + correction * (1.0 / activeCount);
}
