// 多段シャドウの距離区分の選択と境界ブレンド。深度で参照する距離区分を決め、遷移域では2つの距離区分を混ぜる。
// 注意: この式は`crates/blitz_render/src/cascade/blend.rs`の`距離区分を選ぶ`の写しである。
// 単調で和が1になることのユニットテストはRust側が持ち、ここが正本から外れると境界に段差として現れる。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import shadow_sample;

static const uint cascadeCount = 4;

struct CascadeBlend
{
    uint primaryIndex;
    uint secondaryIndex;
    float primaryWeight;
};

// splitDepths.xyzが距離区分の境界、transitionWidths.xyzが境界の手前に置く遷移域の幅である。
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

// 距離区分を選び、遷移域なら2枚のシャドウマップを参照して重みで混ぜる。遷移域の外では1枚しか参照しない。
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

// カメラ相対位置からビュー空間深度(カメラ前方向に沿った距離)を求める。距離区分の分割はこの深度で決まる。
float viewDepthOf(float3 relativePosition, float3 cameraRelativePosition, float3 cameraForward)
{
    return dot(relativePosition - cameraRelativePosition, cameraForward);
}

// 距離区分の可視化色。距離区分ごとに使うチャンネルの組を変え、影の中では同じ色を暗い段へ落とす。
// 検収はチャンネルの組で距離区分を、明るさの段で影の内外を読み取る。2段しか作らないのは、
// 中間の値を作ると読み取り側が距離区分と影を分けられなくなるためである。
float3 cascadeDebugColor(uint cascadeIndex, float shadowFactor)
{
    float3 bandColor = float3(0.0, 0.0, 0.0);
    if (cascadeIndex == 0) bandColor = float3(1.0, 0.0, 0.0);
    else if (cascadeIndex == 1) bandColor = float3(0.0, 1.0, 0.0);
    else if (cascadeIndex == 2) bandColor = float3(0.0, 0.0, 1.0);
    else bandColor = float3(1.0, 1.0, 0.0);
    return bandColor * (shadowFactor > 0.5 ? 1.0 : 0.25);
}

// 可視化する1画素の色。距離区分の選択は本番の色を出す経路と同じ関数を通す。
float3 cascadeDebugPixel(float viewDepth, float3 splitDepths, float3 transitionWidths, float shadowFactor)
{
    return cascadeDebugColor(selectCascadeBlend(viewDepth, splitDepths, transitionWidths).primaryIndex, shadowFactor);
}
