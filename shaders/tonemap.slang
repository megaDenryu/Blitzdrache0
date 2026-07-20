// M7: トーンマップパス(判断38・39)。HDR中間画像(リニア)を全画面三角形でサンプリングし、
// 露出乗算とACES近似トーンカーブで表示リニア0..1へ圧縮して出力する。
// スワップチェーンはSRGB形式のため、sRGBへの符号化はハードウェアが行う。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct ExposurePush
{
    float exposure;
};

[[vk::push_constant]]
ExposurePush exposurePush;

[[vk::binding(0, 0)]] Sampler2D hdrTexture;

struct VertexOutput
{
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
};

[shader("vertex")]
VertexOutput vertexMain(uint vertexID : SV_VertexID)
{
    // 全画面三角形: 頂点ID 0,1,2 をクリップ座標 (-1,-1),(3,-1),(-1,3) に写す。
    // VulkanのNDCはY下向きのため、uv=(0,0)が画面左上・テクスチャ先頭行と一致する。
    float2 corner = float2(float((vertexID << 1) & 2), float(vertexID & 2));
    VertexOutput output;
    output.position = float4(corner * 2.0 - 1.0, 0.0, 1.0);
    output.uv = corner;
    return output;
}

// ACESフィルミックトーンカーブの近似(Narkowiczの定数)。入力はリニアHDR、出力は0..1。
float3 acesFilm(float3 x)
{
    float a = 2.51;
    float b = 0.03;
    float c = 2.43;
    float d = 0.59;
    float e = 0.14;
    return saturate((x * (a * x + b)) / (x * (c * x + d) + e));
}

[shader("fragment")]
float4 fragmentMain(VertexOutput input) : SV_Target
{
    float3 hdr = hdrTexture.Sample(input.uv).rgb;
    float3 exposed = hdr * exposurePush.exposure;
    return float4(acesFilm(exposed), 1.0);
}
