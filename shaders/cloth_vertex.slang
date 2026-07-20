// M9: 布の描画用頂点バッファ生成(判断54)。粒子位置から既存の48バイト頂点レイアウト
// (位置3+法線3+接線4+uv2のfloatフラット12個)を毎フレーム生成する。48バイト互換のため
// シャドウパスは既存シャドウパイプラインのまま布を描ける。
// 法線はグリッド近傍(左右上下)の位置差分の外積、接線は行方向の差分から作る。
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
[[vk::binding(7, 0)]] RWStructuredBuffer<float> clothVertices;

static const uint floatsPerVertex = 12;

float3 particlePosition(uint x, uint y, uint side)
{
    return particles[(y * side + x) * 2].xyz;
}

[shader("compute")]
[numthreads(64, 1, 1)]
void vertexGenMain(uint3 id: SV_DispatchThreadID)
{
    uint index = id.x;
    if (index >= params.counts.x)
    {
        return;
    }
    uint side = params.counts.y;
    uint x = index % side;
    uint y = index / side;

    float3 position = particlePosition(x, y, side);
    // 端の粒子は片側差分になるよう、隣接添字を範囲内へクランプする。
    float3 rowNeighborA = particlePosition(x == 0 ? 0 : x - 1, y, side);
    float3 rowNeighborB = particlePosition(x + 1 >= side ? side - 1 : x + 1, y, side);
    float3 columnNeighborA = particlePosition(x, y == 0 ? 0 : y - 1, side);
    float3 columnNeighborB = particlePosition(x, y + 1 >= side ? side - 1 : y + 1, side);
    float3 rowTangent = rowNeighborB - rowNeighborA;
    float3 columnTangent = columnNeighborB - columnNeighborA;
    float3 crossVector = cross(rowTangent, columnTangent);
    float3 normal = dot(crossVector, crossVector) < 1e-12 ? float3(0.0, 1.0, 0.0) : normalize(crossVector);
    float3 tangent = dot(rowTangent, rowTangent) < 1e-12 ? float3(1.0, 0.0, 0.0) : normalize(rowTangent);

    uint base = index * floatsPerVertex;
    clothVertices[base] = position.x;
    clothVertices[base + 1] = position.y;
    clothVertices[base + 2] = position.z;
    clothVertices[base + 3] = normal.x;
    clothVertices[base + 4] = normal.y;
    clothVertices[base + 5] = normal.z;
    clothVertices[base + 6] = tangent.x;
    clothVertices[base + 7] = tangent.y;
    clothVertices[base + 8] = tangent.z;
    clothVertices[base + 9] = 1.0;
    clothVertices[base + 10] = float(x) / float(side - 1);
    clothVertices[base + 11] = float(y) / float(side - 1);
}
