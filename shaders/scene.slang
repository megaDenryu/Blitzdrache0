// PBR metallic-roughnessモデル(方向光+点光源+環境光+法線マップ)と、方向光の
// シャドウマップ(binding4の比較サンプラー、PCF 3x3)の合成(判断23〜26・35)。
// 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`。シェーダー内識別子はASCII。
// エントリポイント・FrameUniformはここに置き、PBR関数群は`pbr.slang`、
// ライティング合成+シャドウ判定は`lighting.slang`へ分割してimportする。

import pbr;
import lighting;

struct FrameUniform
{
    // column_major明示(CPU側は列優先配列)。lightViewProjectionは先頭(shadow.slangと同じ配置、判断36)。
    column_major float4x4 lightViewProjection;
    column_major float4x4 viewProjection;
    float4 cameraWorldPosition;
    float4 directionalLightDirection;
    float4 directionalLightColor;
    float4 pointLightPosition;
    float4 pointLightColor;
    float4 ambientAndLighting;
    float4 baseColorFactor;
    float4 metallicRoughnessFactor;
};

// binding3: フレームユニフォームバッファ(判断24)。フレームインフライトごとに別のディスクリプタセットが同じbindingを指す。
[[vk::binding(3, 0)]]
ConstantBuffer<FrameUniform> frameUniform;

[[vk::binding(0, 0)]] Sampler2D baseColorTexture;
[[vk::binding(1, 0)]] Sampler2D metallicRoughnessTexture;
[[vk::binding(2, 0)]] Sampler2D normalMapTexture;

// binding4: シャドウマップ(判断35)。テクスチャ+比較サンプラーを同一bindingへ束ねる。
[[vk::binding(4, 0)]] [[vk::combinedImageSampler]] Texture2D shadowMapTexture;
[[vk::binding(4, 0)]] [[vk::combinedImageSampler]] SamplerComparisonState shadowMapSampler;
static const float shadowMapTexelSize = 1.0 / 2048.0;

struct VertexInput
{
    [[vk::location(0)]] float3 position : POSITION;
    [[vk::location(1)]] float3 normal : NORMAL;
    [[vk::location(2)]] float4 tangent : TANGENT;
    [[vk::location(3)]] float2 uv : TEXCOORD0;
};

struct VertexOutput
{
    float4 position : SV_Position;
    float3 worldPosition : TEXCOORD1;
    float3 normal : TEXCOORD2;
    float4 tangent : TEXCOORD3;
    float2 uv : TEXCOORD0;
};

// 注意: この1行の表記はスモークのホットリロード検証が文字列置換で書き換える対象。表記を変えると検証が壊れるため、常にこの形(static const float3 = float3(x, y, z);)を保つ。
static const float3 TINT = float3(1.0, 1.0, 1.0);

[shader("vertex")]
VertexOutput vertexMain(VertexInput input)
{
    // M4ではモデル行列が無い(メッシュはワールドそのまま)ため、位置・法線・接線はそのまま流す。
    VertexOutput output;
    output.position = mul(frameUniform.viewProjection, float4(input.position, 1.0));
    output.worldPosition = input.position;
    output.normal = input.normal;
    output.tangent = input.tangent;
    output.uv = input.uv;
    return output;
}

[shader("fragment")]
float4 fragmentMain(VertexOutput input) : SV_Target
{
    float4 baseColorSample = baseColorTexture.Sample(input.uv);
    float3 albedo = baseColorSample.rgb * frameUniform.baseColorFactor.rgb * TINT;

    bool lightingEnabled = frameUniform.ambientAndLighting.y > 0.5;
    if (!lightingEnabled)
    {
        return float4(albedo, 1.0);
    }

    // glTF規約: metallicRoughnessテクスチャはG=粗さ・B=金属度。
    float2 metallicRoughnessSample = metallicRoughnessTexture.Sample(input.uv).gb;
    float3 sampledNormal = normalMapTexture.Sample(input.uv).rgb * 2.0 - 1.0;
    float shadowFactor = computeShadowFactor(
        input.worldPosition, frameUniform.lightViewProjection,
        shadowMapTexture, shadowMapSampler, shadowMapTexelSize);

    float3 litColor = shadeLitSurface(
        albedo, metallicRoughnessSample,
        frameUniform.metallicRoughnessFactor.x, frameUniform.metallicRoughnessFactor.y,
        sampledNormal, input.normal, input.tangent,
        input.worldPosition, frameUniform.cameraWorldPosition.xyz, frameUniform.ambientAndLighting.x,
        frameUniform.directionalLightDirection.xyz, frameUniform.directionalLightColor.rgb, frameUniform.directionalLightColor.w,
        frameUniform.pointLightPosition.xyz, frameUniform.pointLightColor.rgb, frameUniform.pointLightColor.w,
        shadowFactor);

    return float4(litColor, 1.0);
}
