// ブルーム拡大チェーン(判断41): 1段小さい解像度の拡大結果を3x3テントで広げ、
// 同解像度の縮小結果へ加算して書く。フィルタ本体はbloom_filters.slangにある。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import bloom_filters;

// 1段小さい解像度の拡大チェーン結果(最下段では縮小チェーン最終段)。
[[vk::binding(0, 0)]] Sampler2D smallerTexture;
// 出力と同解像度の縮小チェーン結果。
[[vk::binding(1, 0)]] Sampler2D baseTexture;

struct VertexOutput
{
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
};

[shader("vertex")]
VertexOutput vertexMain(uint vertexID : SV_VertexID)
{
    // 全画面三角形(tonemap.slangと同じcorner写像)。
    float2 corner = float2(float((vertexID << 1) & 2), float(vertexID & 2));
    VertexOutput output;
    output.position = float4(corner * 2.0 - 1.0, 0.0, 1.0);
    output.uv = corner;
    return output;
}

[shader("fragment")]
float4 upsampleMain(VertexOutput input) : SV_Target
{
    uint width;
    uint height;
    smallerTexture.GetDimensions(width, height);
    float2 texel = float2(1.0 / float(width), 1.0 / float(height));
    float3 up = tent9(smallerTexture, input.uv, texel);
    float3 base = baseTexture.Sample(input.uv).rgb;
    return float4(base + up, 1.0);
}
