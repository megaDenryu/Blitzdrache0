// PBR関数群: GGX分布・幾何項(Smith)・フレネル(Schlick)・法線マップ適用・
// Cook-TorranceのGGX鏡面反射+Lambert拡散のシェーディング(判断25〜26)。
// scene.slangから`import pbr;`で使う。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

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
