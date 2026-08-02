// シャドウマップの1つの距離区分をPCF 3x3で比較サンプリングする工程。受け取るのはカメラ相対位置と
// その距離区分のライトビュー射影、返すのは影係数(1=非シャドウ、0=シャドウ)である。
// 距離区分の選択と境界の混合は`cascade_shadow.slang`、直接光の合成は`direct_light_shading.slang`が持つ。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// texelSizeは呼び出し側がシャドウマップ解像度から渡し、cascadeIndexが配列テクスチャの層を選ぶ。
// 位置とライトビュー射影は同じカメラ相対フレームで与える(前提: 定数の書き込みが同じ原点を引く)。
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
