// M6: シーンの最終ライティング合成とシャドウ判定。scene.slangのフラグメント本体を
// ここへ切り出し、100行制限内に収める(pbr.slangのGGX等はここから呼ぶ)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import pbr;

// カスケードシャドウマップの1つの距離区分をPCF 3x3で比較サンプリングし、影係数(1=非シャドウ,0=シャドウ)を返す。
// texelSizeは呼び出し側(scene.slang)がシャドウマップ解像度から渡し、cascadeIndexが配列テクスチャの層を選ぶ。
// 位置とライトビュー射影は同じカメラ相対フレームで与える(前提: フレームUBOの書き込みが同じ原点を引く)。
float computeShadowFactor(
    float3 relativePosition, float4x4 lightViewProjection, uint cascadeIndex,
    Texture2DArray shadowMapTexture, SamplerComparisonState shadowMapSampler, float texelSize)
{
    float4 lightClipPosition = mul(lightViewProjection, float4(relativePosition, 1.0));
    float3 lightNdc = lightClipPosition.xyz / lightClipPosition.w;
    float2 shadowUv = lightNdc.xy * 0.5 + 0.5;
    float currentDepth = lightNdc.z;

    // 影の正射影範囲の外は影の情報を持たない。サンプラーのクランプ結果を使うと範囲外が一律に暗くなるため、影なしとして返す。
    if (any(shadowUv < 0.0) || any(shadowUv > 1.0) || currentDepth < 0.0 || currentDepth > 1.0)
    {
        return 1.0;
    }

    float shadowSum = 0.0;
    for (int offsetY = -1; offsetY <= 1; offsetY++)
    {
        for (int offsetX = -1; offsetX <= 1; offsetX++)
        {
            float2 sampleUv = shadowUv + float2(offsetX, offsetY) * texelSize;
            shadowSum += shadowMapTexture.SampleCmp(shadowMapSampler, float3(sampleUv, float(cascadeIndex)), currentDepth);
        }
    }
    return shadowSum / 9.0;
}

// ambient/directional/pointの合成(判断26)。directional項にのみshadowFactorを乗じる
// (判断35: 影は方向光の寄与にのみ適用する)。
float3 shadeLitSurface(
    float3 albedo, float2 metallicRoughnessSample,
    float metallicFactor, float roughnessFactor,
    float3 sampledNormal, float3 inputNormal, float4 inputTangent,
    float3 relativePosition, float3 cameraRelativePosition, float ambientFactor,
    float3 directionalLightDirection, float3 directionalLightColor, float directionalLightIntensity,
    float3 pointLightPosition, float3 pointLightColor, float pointLightIntensity,
    float shadowFactor)
{
    float roughness = saturate(metallicRoughnessSample.x * roughnessFactor);
    float metallic = saturate(metallicRoughnessSample.y * metallicFactor);
    float3 normal = applyNormalMap(inputNormal, inputTangent, sampledNormal);
    float3 viewDir = normalize(cameraRelativePosition - relativePosition);

    float3 ambientTerm = albedo * ambientFactor;

    float3 lightDir = normalize(-directionalLightDirection);
    float3 radiance = directionalLightColor * directionalLightIntensity;
    float3 directionalTerm = shadeLight(normal, viewDir, lightDir, radiance, albedo, metallic, roughness) * shadowFactor;

    float3 toPoint = pointLightPosition - relativePosition;
    float pointDistance = max(length(toPoint), 1e-4);
    float3 pointDir = toPoint / pointDistance;
    float3 pointRadiance = pointLightColor * pointLightIntensity / (pointDistance * pointDistance);
    float3 pointTerm = shadeLight(normal, viewDir, pointDir, pointRadiance, albedo, metallic, roughness);

    return ambientTerm + directionalTerm + pointTerm;
}
