// スカイビューの積分が使う補助: 積分の座標系の組み立て、位相関数の組、多重散乱LUTの標本取り。
// 注意: これらの式は`crates/blitz_render/src/atmosphere`の写しである(skyview_lut.rsの幾何の組み立て、
// skyview_march.rsの位相の組、skyview_step.rsの多重散乱の引き方)。正本はRust側にある。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_medium;
import atmosphere_multiscatter_mapping;
import atmosphere_skyview_mapping;

struct SkyViewGeometry
{
    float3 observerPosition;
    float3 viewDirection;
    float3 sunDirection;
};

// 天頂を+Zに取り、太陽をXZ平面の正のX側へ置いた座標系を組む。視線のY成分を常に非負に取るのは、
// 球対称の大気では太陽をまたぐ方位の符号が結果を変えないためである。
SkyViewGeometry skyViewGeometryOf(float observerRadius, float sunZenithCosine, SkyViewRay ray)
{
    float sunLateral = sqrt(max(0.0, 1.0 - sunZenithCosine * sunZenithCosine));
    float viewLateral = sqrt(max(0.0, 1.0 - ray.viewZenithCosine * ray.viewZenithCosine));
    float azimuthSine = sqrt(max(0.0, 1.0 - ray.sunAzimuthCosine * ray.sunAzimuthCosine));

    SkyViewGeometry geometry;
    geometry.observerPosition = float3(0.0, 0.0, observerRadius);
    geometry.viewDirection = float3(viewLateral * ray.sunAzimuthCosine, viewLateral * azimuthSine, ray.viewZenithCosine);
    geometry.sunDirection = float3(sunLateral, 0.0, sunZenithCosine);
    return geometry;
}

// 空気分子と浮遊粒子の位相関数値の組。散乱角の余弦は視線と太陽の向きの内積である。
// 入ってくる光子の向きは太陽の逆、出ていく向きは視線の逆であり、2つの逆向きの内積は元の2つの内積に等しい
// (参照: Bruneton 2017実装の`GetSkyRadiance`の`nu`)。xがレイリー、yがミー(Cornette-Shanks形)である。
float2 skyViewPhasePair(AtmosphereMedium medium, SkyViewGeometry geometry)
{
    float cosine = clamp(dot(geometry.viewDirection, geometry.sunDirection), -1.0, 1.0);
    float rayleigh = 3.0 / (16.0 * atmospherePi) * (1.0 + cosine * cosine);
    float g = medium.groundAlbedoAndAsymmetry.w;
    float factor = 3.0 / (8.0 * atmospherePi) * (1.0 - g * g) / (2.0 + g * g);
    // 非対称係数の絶対値が1未満であることを媒体の不変条件が保証するため、この分母は正のままである。
    float denominator = pow(1.0 + g * g - 2.0 * g * cosine, 1.5);
    return float2(rayleigh, factor * (1.0 + cosine * cosine) / denominator);
}

// その位置の多重散乱の伝達率。太陽天頂余弦と半径から単位位置を求め、テクセル中心のUVへ直して引く。
float3 sampleMultiScatter(AtmosphereMedium medium, Sampler2D multiScatterLut, float3 position, float3 sunDirection)
{
    float radius = length(position);
    if (radius <= 0.0)
    {
        return float3(0.0, 0.0, 0.0);
    }
    uint width;
    uint height;
    multiScatterLut.GetDimensions(width, height);
    float sunZenithCosine = clamp(dot(position / radius, sunDirection), -1.0, 1.0);
    float2 unitPosition = multiScatterUnitFromCondition(medium, sunZenithCosine, radius);
    return multiScatterLut.SampleLevel(texelCenterUvFromUnit(unitPosition, uint2(width, height)), 0.0).rgb;
}
