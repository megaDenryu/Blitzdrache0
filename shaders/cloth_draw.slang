// M9: 布の描画(判断54)。両面ランバート+方向光シャドウの最小シェーダー。
// ディスクリプタはシーンと同じセット(binding3=フレームユニフォーム、binding4=シャドウマップ)を
// 束縛し、PBRテクスチャ(binding0〜2)は読まない。頂点レイアウトはscene.slangと同じ48バイト。
// 前提: FrameUniformのレイアウトはscene.slangと完全一致させること(同じUBOを読むため)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct FrameUniform
{
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

[[vk::binding(3, 0)]]
ConstantBuffer<FrameUniform> frameUniform;

[[vk::binding(4, 0)]] [[vk::combinedImageSampler]] Texture2D shadowMapTexture;
[[vk::binding(4, 0)]] [[vk::combinedImageSampler]] SamplerComparisonState shadowMapSampler;

static const float3 clothColor = float3(0.55, 0.1, 0.12);

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
};

[shader("vertex")]
VertexOutput vertexMain(VertexInput input)
{
    VertexOutput output;
    output.position = mul(frameUniform.viewProjection, float4(input.position, 1.0));
    output.worldPosition = input.position;
    output.normal = input.normal;
    return output;
}

float shadowFactor(float3 worldPosition)
{
    float4 lightClip = mul(frameUniform.lightViewProjection, float4(worldPosition, 1.0));
    float3 projected = lightClip.xyz / lightClip.w;
    float2 shadowUv = projected.xy * float2(0.5, -0.5) + 0.5;
    if (any(shadowUv < 0.0) || any(shadowUv > 1.0))
    {
        return 1.0;
    }
    return shadowMapTexture.SampleCmp(shadowMapSampler, shadowUv, projected.z - 0.002);
}

// 両面ランバート: 裏面から見た場合は法線を反転して照らす(布は薄い面のため)。
[shader("fragment")]
float4 fragmentMain(VertexOutput input, bool isFrontFace: SV_IsFrontFace) : SV_Target
{
    float3 normal = normalize(input.normal) * (isFrontFace ? 1.0 : -1.0);
    float3 toLight = -normalize(frameUniform.directionalLightDirection.xyz);
    float lambert = max(dot(normal, toLight), 0.0);
    float shadow = shadowFactor(input.worldPosition);
    float3 direct = frameUniform.directionalLightColor.rgb * lambert * shadow;
    float3 ambient = float3(0.15, 0.15, 0.15);
    return float4(clothColor * (direct + ambient), 1.0);
}
