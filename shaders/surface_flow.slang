// 表面流のM10技術検証。128x128のUV格子を縦方向だけ保存的に移流する。
// 1列を1スレッドが逐次処理するため、同一バッファ更新でも競合しない。
// 注意: 横方向移流はCPU参照計算だけで検証し、このGPU試作の対象外とする。

import relative_anchor;
import view_uniform;

struct SurfaceCell
{
    float4 state;
    float4 reserved;
};

[[vk::binding(3, 0)]]
RWStructuredBuffer<SurfaceCell> cellsRW;
[[vk::binding(3, 0)]]
StructuredBuffer<SurfaceCell> cellsRO;

static const uint GRID_SIZE = 128;
static const float DELTA_TIME = 1.0 / 60.0;

[shader("compute")]
[numthreads(64, 1, 1)]
void computeMain(uint3 id : SV_DispatchThreadID)
{
    if (id.x >= GRID_SIZE) return;
    for (uint row = 1; row < GRID_SIZE; ++row)
    {
        uint index = row * GRID_SIZE + id.x;
        uint belowIndex = index - GRID_SIZE;
        SurfaceCell cell = cellsRW[index];
        SurfaceCell below = cellsRW[belowIndex];
        cell.state.z = cell.state.z * 0.99 - 1.5 * DELTA_TIME;
        float transfer = min(cell.state.x, max(0.0, -cell.state.z) * DELTA_TIME * cell.state.x);
        cell.state.x -= transfer;
        below.state.x += transfer;
        cellsRW[index] = cell;
        cellsRW[belowIndex] = below;
    }
}

struct VertexOutput
{
    float4 position : SV_Position;
    float pointSize : SV_PointSize;
    float thickness : TEXCOORD0;
};

[shader("vertex")]
VertexOutput vertexMain(uint vertexID : SV_VertexID)
{
    uint column = vertexID % GRID_SIZE;
    uint row = vertexID / GRID_SIZE;
    float thickness = cellsRO[vertexID].state.x;
    float2 uv = float2(column, row) / float(GRID_SIZE - 1);
    VertexOutput output;
    float3 relativePosition = applyRelativeAnchor(float3((uv.x - 0.5) * 4.0, uv.y * 4.0 - 1.0, 0.3));
    output.position = mul(viewUniform.viewProjection, float4(relativePosition, 1.0));
    output.pointSize = thickness > 0.001 ? 5.0 : 0.0;
    output.thickness = thickness;
    return output;
}

[shader("fragment")]
float4 fragmentMain(VertexOutput input) : SV_Target
{
    return float4(0.05, 0.35 + min(input.thickness, 1.0) * 0.4, 1.0, 1.0);
}
