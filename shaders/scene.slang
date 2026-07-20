// M4: PBR metallic-roughnessモデル。方向光+点光源+環境光。法線マップ(TBN)。
// 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断23」〜「判断26」。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。
// 注意: この1ファイルに留める理由: ホットリロード(blitz_app::hot_reload::compile)が
// scene.slang単体を一時ディレクトリへコピーして再コンパイルするため、import分割すると
// 分割先ファイルが一時ディレクトリに存在せずビルドが壊れる。モジュール分割は
// ホットリロードのコピー機構(xtask smoke::copy_setupも同様に単一ファイル前提)を
// 拡張してから行う。

struct FrameUniform
{
    // column_major明示: CPU側は列優先の生配列(blitz_mathのgpu境界用列優先配列)で
    // 送るため、シェーダー側の解釈をそれに一致させる。
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

// binding3: フレームユニフォームバッファ(判断24)。フレームインフライトごとに
// 別のディスクリプタセットが同じbindingを指す。
[[vk::binding(3, 0)]]
ConstantBuffer<FrameUniform> frameUniform;

[[vk::binding(0, 0)]]
Sampler2D baseColorTexture;
[[vk::binding(1, 0)]]
Sampler2D metallicRoughnessTexture;
[[vk::binding(2, 0)]]
Sampler2D normalMapTexture;

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

// 注意: この1行の表記はスモークのホットリロード検証が文字列置換で書き換える対象。
// 表記を変えると検証が壊れるため、常にこの形(static const float3 = float3(x, y, z);)を保つ。
static const float3 TINT = float3(1.0, 1.0, 1.0);

[shader("vertex")]
VertexOutput vertexMain(VertexInput input)
{
    // M4ではモデル行列が無い(メッシュはワールドそのまま)ため、位置・法線・接線は
    // そのまま流す。
    VertexOutput output;
    output.position = mul(frameUniform.viewProjection, float4(input.position, 1.0));
    output.worldPosition = input.position;
    output.normal = input.normal;
    output.tangent = input.tangent;
    output.uv = input.uv;
    return output;
}

// TBN基底を直接の線形結合で組み立てる(行列の行/列規約の曖昧さを避けるため
// float3x3を経由しない)。tangent.wは従法線の向き(±1、判断25)。
float3 applyNormalMap(float3 normal, float4 tangent, float3 sampledNormal)
{
    float3 n = normalize(normal);
    float3 t = normalize(tangent.xyz - n * dot(n, tangent.xyz));
    float3 b = cross(n, t) * tangent.w;
    return normalize(sampledNormal.x * t + sampledNormal.y * b + sampledNormal.z * n);
}

float distributionGGX(float nDotH, float roughness)
{
    float a = roughness * roughness;
    float a2 = a * a;
    float denom = nDotH * nDotH * (a2 - 1.0) + 1.0;
    return a2 / max(3.14159265 * denom * denom, 1e-6);
}

float geometrySchlickGGX(float nDotV, float roughness)
{
    float k = (roughness + 1.0) * (roughness + 1.0) / 8.0;
    return nDotV / max(nDotV * (1.0 - k) + k, 1e-6);
}

float geometrySmith(float nDotV, float nDotL, float roughness)
{
    return geometrySchlickGGX(nDotV, roughness) * geometrySchlickGGX(nDotL, roughness);
}

float3 fresnelSchlick(float cosTheta, float3 f0)
{
    float p = pow(saturate(1.0 - cosTheta), 5.0);
    return f0 + (1.0 - f0) * p;
}

// Cook-TorranceのGGX鏡面反射+Lambert拡散(判断26)。radianceは減衰込みの入射光。
float3 shadeLight(
    float3 normal, float3 viewDir, float3 lightDir, float3 radiance,
    float3 albedo, float metallic, float roughness)
{
    float3 halfDir = normalize(viewDir + lightDir);
    float nDotV = max(dot(normal, viewDir), 0.0);
    float nDotL = max(dot(normal, lightDir), 0.0);
    float nDotH = max(dot(normal, halfDir), 0.0);
    float hDotV = max(dot(halfDir, viewDir), 0.0);

    float3 f0 = lerp(float3(0.04, 0.04, 0.04), albedo, metallic);
    float d = distributionGGX(nDotH, roughness);
    float g = geometrySmith(nDotV, nDotL, roughness);
    float3 f = fresnelSchlick(hDotV, f0);

    float3 specular = (d * g * f) / max(4.0 * nDotV * nDotL, 1e-6);
    float3 kd = (float3(1.0, 1.0, 1.0) - f) * (1.0 - metallic);
    float3 diffuse = kd * albedo / 3.14159265;

    return (diffuse + specular) * radiance * nDotL;
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
    float3 directionalRadiance =
        frameUniform.directionalLightColor.rgb * frameUniform.directionalLightColor.w;
    float3 directionalTerm =
        shadeLight(normal, viewDir, directionalLightDir, directionalRadiance, albedo, metallic, roughness);

    float3 toPointLight = frameUniform.pointLightPosition.xyz - input.worldPosition;
    float pointLightDistance = max(length(toPointLight), 1e-4);
    float3 pointLightDir = toPointLight / pointLightDistance;
    float3 pointRadiance = frameUniform.pointLightColor.rgb * frameUniform.pointLightColor.w
        / (pointLightDistance * pointLightDistance);
    float3 pointTerm =
        shadeLight(normal, viewDir, pointLightDir, pointRadiance, albedo, metallic, roughness);

    return float4(ambientTerm + directionalTerm + pointTerm, 1.0);
}
