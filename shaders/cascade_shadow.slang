// カスケードシャドウの帯選択と境界ブレンド。深度で引く帯を決め、遷移帯では2つの帯を混ぜる。
// 注意: この式は`crates/blitz_render/src/cascade/blend.rs`の`帯を選ぶ`の写しである。
// 単調で和が1になることのユニットテストはRust側が持ち、ここが正本から外れると境界に段差として現れる。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import lighting;

static const uint cascadeCount = 4;

struct CascadeBlend
{
    uint primaryIndex;
    uint secondaryIndex;
    float primaryWeight;
};

// splitDepths.xyzが帯の境界、transitionWidths.xyzが境界の手前に置く遷移帯の幅である。
CascadeBlend selectCascadeBlend(float viewDepth, float3 splitDepths, float3 transitionWidths)
{
    CascadeBlend blend;
    for (uint index = 0; index < cascadeCount - 1; ++index)
    {
        float boundary = splitDepths[index];
        if (viewDepth > boundary)
        {
            continue;
        }
        float width = transitionWidths[index];
        float transitionStart = boundary - width;
        blend.primaryIndex = index;
        if (width > 0.0 && viewDepth >= transitionStart)
        {
            blend.secondaryIndex = index + 1;
            blend.primaryWeight = 1.0 - saturate((viewDepth - transitionStart) / width);
            return blend;
        }
        blend.secondaryIndex = index;
        blend.primaryWeight = 1.0;
        return blend;
    }
    blend.primaryIndex = cascadeCount - 1;
    blend.secondaryIndex = cascadeCount - 1;
    blend.primaryWeight = 1.0;
    return blend;
}

// 帯を選び、遷移帯なら2枚のシャドウマップを引いて重みで混ぜる。遷移帯の外では1枚しか引かない。
float computeCascadedShadowFactor(
    float3 relativePosition, float viewDepth,
    float4x4 cascadeLightViewProjection[cascadeCount], float3 splitDepths, float3 transitionWidths,
    Texture2DArray shadowMapTexture, SamplerComparisonState shadowMapSampler, float texelSize)
{
    CascadeBlend blend = selectCascadeBlend(viewDepth, splitDepths, transitionWidths);
    float primary = computeShadowFactor(
        relativePosition, cascadeLightViewProjection[blend.primaryIndex], blend.primaryIndex,
        shadowMapTexture, shadowMapSampler, texelSize);
    if (blend.secondaryIndex == blend.primaryIndex)
    {
        return primary;
    }
    float secondary = computeShadowFactor(
        relativePosition, cascadeLightViewProjection[blend.secondaryIndex], blend.secondaryIndex,
        shadowMapTexture, shadowMapSampler, texelSize);
    return lerp(secondary, primary, blend.primaryWeight);
}

// カメラ相対位置からビュー空間深度(カメラ前方向に沿った距離)を求める。帯の分割はこの深度で決まる。
float viewDepthOf(float3 relativePosition, float3 cameraRelativePosition, float3 cameraForward)
{
    return dot(relativePosition - cameraRelativePosition, cameraForward);
}
