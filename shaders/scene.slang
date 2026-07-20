// M4: PBR metallic-roughnessモデル。方向光+点光源+環境光。法線マップ(TBN)。
// 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断23」〜「判断26」。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。
// 構成: エントリポイント・入出力構造体・FrameUniform/バインディング宣言はここに置く。
// PBR関数群(GGX分布・幾何項・フレネル・shadeLight・applyNormalMap)は`pbr.slang`へ分割し、
// `import pbr;`で使う(hot_reload::compileとsmoke::copy_setupがshaders/ディレクトリ
// 単位でコピー・監視するため分割可能になった)。

import pbr;

struct FrameUniform
{
    // column_major明示: CPU側は列優先の生配列(blitz_mathのgpu境界用列優先配列)で送るため、シェーダー側の解釈をそれに一致させる。
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
    float roughness = saturate(metallicRoughnessSample.x * frameUniform.metallicRoughnessFactor.y);
    float metallic = saturate(metallicRoughnessSample.y * frameUniform.metallicRoughnessFactor.x);

    float3 sampledNormal = normalMapTexture.Sample(input.uv).rgb * 2.0 - 1.0;
    float3 normal = applyNormalMap(input.normal, input.tangent, sampledNormal);
    float3 viewDir = normalize(frameUniform.cameraWorldPosition.xyz - input.worldPosition);

    float3 ambientTerm = albedo * frameUniform.ambientAndLighting.x;

    float3 directionalLightDir = normalize(-frameUniform.directionalLightDirection.xyz);
    float3 directionalRadiance = frameUniform.directionalLightColor.rgb * frameUniform.directionalLightColor.w;
    float3 directionalTerm = shadeLight(normal, viewDir, directionalLightDir, directionalRadiance, albedo, metallic, roughness);

    float3 toPointLight = frameUniform.pointLightPosition.xyz - input.worldPosition;
    float pointLightDistance = max(length(toPointLight), 1e-4);
    float3 pointLightDir = toPointLight / pointLightDistance;
    float3 pointRadiance = frameUniform.pointLightColor.rgb * frameUniform.pointLightColor.w / (pointLightDistance * pointLightDistance);
    float3 pointTerm = shadeLight(normal, viewDir, pointLightDir, pointRadiance, albedo, metallic, roughness);

    return float4(ambientTerm + directionalTerm + pointTerm, 1.0);
}
