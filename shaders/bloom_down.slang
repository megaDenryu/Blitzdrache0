// ブルーム縮小チェーン(判断41): 前処理(初段)と2段目以降の縮小。
// 前処理はHDR画像を読み、Karis平均付き13タップ縮小+ソフト閾値で1/2解像度へ抽出する。
// 2段目以降は13タップ縮小のみ。フィルタ本体はbloom_filters.slangにある。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import bloom_filters;

[[vk::binding(0, 0)]] Sampler2D sourceTexture;

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

float2 sourceTexel()
{
    uint width;
    uint height;
    sourceTexture.GetDimensions(width, height);
    return float2(1.0 / float(width), 1.0 / float(height));
}

[shader("fragment")]
float4 prefilterMain(VertexOutput input) : SV_Target
{
    float3 color = downsample13Karis(sourceTexture, input.uv, sourceTexel());
    return float4(softThreshold(color), 1.0);
}

[shader("fragment")]
float4 downsampleMain(VertexOutput input) : SV_Target
{
    return float4(downsample13(sourceTexture, input.uv, sourceTexel()), 1.0);
}
