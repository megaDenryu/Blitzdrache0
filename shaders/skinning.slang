// M8: GPUスキニング(判断44)。レスト姿勢の頂点をスキン行列(ジョイントごとの
// ローカル→アニメ後ローカル変換)のウェイト付き合成で変形し、描画用頂点バッファへ書く。
// 頂点はRust側のrepr(C)密パック(48バイト=float12個)のため、float3のstd430アラインメント
// 差異を避けてfloat単位のフラット配列として読み書きする。
// 行列は列優先のfloat4を4本/ジョイントで受け、掛け算を列ベクトルの線形結合で書く(レイアウト非依存)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct SkinPush
{
    uint vertexCount;
};

[[vk::push_constant]]
SkinPush skinPush;

[[vk::binding(0, 0)]] StructuredBuffer<float> restVertices;
struct SkinAttribute
{
    uint4 joints;
    float4 weights;
};
[[vk::binding(1, 0)]] StructuredBuffer<SkinAttribute> skinAttributes;
// 列優先: ジョイントjの行列は matrixColumns[j*4+0..j*4+3] の4列。
[[vk::binding(2, 0)]] StructuredBuffer<float4> matrixColumns;
[[vk::binding(3, 0)]] RWStructuredBuffer<float> skinnedVertices;

static const uint floatsPerVertex = 12;

float3 transformPoint(uint jointIndex, float3 p)
{
    uint base = jointIndex * 4;
    float4 r = matrixColumns[base] * p.x + matrixColumns[base + 1] * p.y + matrixColumns[base + 2] * p.z + matrixColumns[base + 3];
    return r.xyz;
}

// 前提: スキン行列のスケールは均一(判断44。非均一スケールの逆転置は対象外)。
float3 transformDirection(uint jointIndex, float3 d)
{
    uint base = jointIndex * 4;
    return (matrixColumns[base] * d.x + matrixColumns[base + 1] * d.y + matrixColumns[base + 2] * d.z).xyz;
}

[shader("compute")]
[numthreads(64, 1, 1)]
void computeMain(uint3 dispatchThreadID: SV_DispatchThreadID)
{
    uint index = dispatchThreadID.x;
    if (index >= skinPush.vertexCount)
    {
        return;
    }
    uint base = index * floatsPerVertex;
    float3 position = float3(restVertices[base], restVertices[base + 1], restVertices[base + 2]);
    float3 normal = float3(restVertices[base + 3], restVertices[base + 4], restVertices[base + 5]);
    float3 tangent = float3(restVertices[base + 6], restVertices[base + 7], restVertices[base + 8]);

    SkinAttribute attribute = skinAttributes[index];
    float3 skinnedPosition = float3(0.0, 0.0, 0.0);
    float3 skinnedNormal = float3(0.0, 0.0, 0.0);
    float3 skinnedTangent = float3(0.0, 0.0, 0.0);
    for (int i = 0; i < 4; i++)
    {
        float weight = attribute.weights[i];
        if (weight == 0.0)
        {
            continue;
        }
        uint jointIndex = attribute.joints[i];
        skinnedPosition += transformPoint(jointIndex, position) * weight;
        skinnedNormal += transformDirection(jointIndex, normal) * weight;
        skinnedTangent += transformDirection(jointIndex, tangent) * weight;
    }
    skinnedNormal = normalize(skinnedNormal);
    skinnedTangent = normalize(skinnedTangent);

    skinnedVertices[base] = skinnedPosition.x;
    skinnedVertices[base + 1] = skinnedPosition.y;
    skinnedVertices[base + 2] = skinnedPosition.z;
    skinnedVertices[base + 3] = skinnedNormal.x;
    skinnedVertices[base + 4] = skinnedNormal.y;
    skinnedVertices[base + 5] = skinnedNormal.z;
    skinnedVertices[base + 6] = skinnedTangent.x;
    skinnedVertices[base + 7] = skinnedTangent.y;
    skinnedVertices[base + 8] = skinnedTangent.z;
    // 接線w(従法線の向き)とuvは変形しない。
    skinnedVertices[base + 9] = restVertices[base + 9];
    skinnedVertices[base + 10] = restVertices[base + 10];
    skinnedVertices[base + 11] = restVertices[base + 11];
}
