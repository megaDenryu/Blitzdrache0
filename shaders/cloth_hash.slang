// M9: セルフコリジョン用の空間グリッド構築(判断52・54)。セル寸法=粒子間隔、32^3セル・
// セル容量8のu32格納。格納はセルカウントの原子加算、あふれた粒子は無視する(初版の明示的制約)。
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
static const uint cellTotal = gridSide * gridSide * gridSide;

[shader("compute")]
[numthreads(64, 1, 1)]
void hashClearMain(uint3 id: SV_DispatchThreadID)
{
    if (id.x < cellTotal)
    {
        cellCounts[id.x] = 0;
    }
}

// 粒子をセルへ格納する。グリッド範囲外の粒子はセルフコリジョン対象から外れる(初版の明示的制約)。
[shader("compute")]
[numthreads(64, 1, 1)]
void hashStoreMain(uint3 id: SV_DispatchThreadID)
{
    uint index = id.x;
    if (index >= params.counts.x)
    {
        return;
    }
    float3 relative = (particles[index * 2].xyz - params.gridOrigin.xyz) / params.gridOrigin.w;
    int3 cell = int3(floor(relative));
    if (any(cell < int3(0, 0, 0)) || any(cell >= int3(gridSide, gridSide, gridSide)))
    {
        return;
    }
    uint cellIndex = (uint(cell.z) * gridSide + uint(cell.y)) * gridSide + uint(cell.x);
    uint slot;
    InterlockedAdd(cellCounts[cellIndex], 1, slot);
    if (slot < cellCapacity)
    {
        cellSlots[cellIndex * cellCapacity + slot] = index;
    }
}
