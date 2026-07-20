// 開発用UI(egui)描画(M5波3判断33)。頂点色・テクスチャはsRGB空間の値のため、
// リニアへ変換してから合成する。スワップチェーンはSRGB形式でシェーダー出力を
// リニア→sRGBへハードウェアが自動エンコードするため、ここではリニア空間で
// premultiplied alphaの値を返せばよい(egui公式レンダラーの変換方針と同じ)。
// 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断33」。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct ScreenSize
{
    float2 sizeInPixels;
};

[[vk::push_constant]]
ScreenSize screenSize;

[[vk::binding(0, 0)]] Sampler2D uiTexture;

struct VertexInput
{
    [[vk::location(0)]] float2 positionPx : POSITION;
    [[vk::location(1)]] float2 uv : TEXCOORD0;
    [[vk::location(2)]] float4 color : COLOR0;
};

struct VertexOutput
{
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
    float4 colorLinear : COLOR0;
};

float3 srgbToLinear(float3 srgb)
{
    float3 lower = srgb / 12.92;
    float3 higher = pow((srgb + 0.055) / 1.055, 2.4);
    float3 cutoff = step(float3(0.04045, 0.04045, 0.04045), srgb);
    return lerp(lower, higher, cutoff);
}

[shader("vertex")]
VertexOutput vertexMain(VertexInput input)
{
    VertexOutput output;
    float2 clipPosition = (input.positionPx / screenSize.sizeInPixels) * 2.0 - 1.0;
    output.position = float4(clipPosition.x, clipPosition.y, 0.0, 1.0);
    output.uv = input.uv;
    // アルファは線形補間で問題ない(ガンマ曲線を持たない量のため)。
    output.colorLinear = float4(srgbToLinear(input.color.rgb), input.color.a);
    return output;
}

[shader("fragment")]
float4 fragmentMain(VertexOutput input) : SV_Target
{
    float4 textureSample = uiTexture.Sample(input.uv);
    float4 textureLinear = float4(srgbToLinear(textureSample.rgb), textureSample.a);
    // 両者ともpremultiplied alphaのため、成分ごとの積がpremultiplied合成結果になる
    // (egui公式レンダラーと同じ方針)。
    return input.colorLinear * textureLinear;
}
