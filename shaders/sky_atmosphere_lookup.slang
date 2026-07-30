// 空パスが大気LUTを引く工程。受け取るのはワールドの視線と観測半径、返すのはスカイビューLUTの放射輝度と
// 太陽方向の透過率である。引くための資源(媒体のユニフォームと2枚のLUT)もここが宣言する。
// 注意: 写像の正本は`crates/blitz_render/src/atmosphere`(skyview_lookup.rs・skyview_mapping.rs・
// transmittance_mapping.rs・intersect.rs)にあり、ここはその写しである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_mapping;
import atmosphere_medium;
import atmosphere_skyview_mapping;

// set1は空パスが大気LUTを引くためだけのディスクリプタである。媒体のユニフォームを持つのは、LUTの写像が
// 惑星下端半径と大気上端半径を要るためである。2枚のLUTのレイアウトはGENERALであり、レンダーグラフの
// 画像用途「大気LUTフラグメント読み」が同じレイアウトを指す。
[[vk::binding(0, 1)]] ConstantBuffer<AtmosphereMedium> atmosphereMedium;
[[vk::binding(1, 1)]] Sampler2D transmittanceLut;
[[vk::binding(2, 1)]] Sampler2D skyViewLut;

// 水平成分がこの長さ未満なら方位を決められないとみなす。天頂と天底の行はどの横位置でも同じ値であり、
// 天頂の太陽はどの方位から見ても対称であるため、この場合にどの方位を選んでも引く値は変わらない。
// 測るのは`length`(ユークリッドの長さ)であり、正本の`方位を決められない水平成分の長さ`と同じノルムである。
// 正本は`crates/blitz_render/src/atmosphere/skyview_lookup.rs`にある。
static const float skyAzimuthEpsilon = 1.0e-6;

// 天頂を+Yに取ったワールドの視線と太陽方向から、LUTの縦横が意味する視線を求める。
// 方位余弦を内積そのものでなく水平面へ落としてから取るのは、横軸が太陽との相対方位であって角度ではないためである。
SkyViewRay skyViewRayOfWorldDirection(float3 viewDirection, float3 sunDirection)
{
    float2 viewHorizontal = float2(viewDirection.x, viewDirection.z);
    float2 sunHorizontal = float2(sunDirection.x, sunDirection.z);
    float azimuthCosine = 1.0;
    if (length(viewHorizontal) >= skyAzimuthEpsilon && length(sunHorizontal) >= skyAzimuthEpsilon)
    {
        azimuthCosine = clamp(dot(normalize(viewHorizontal), normalize(sunHorizontal)), -1.0, 1.0);
    }

    SkyViewRay ray;
    ray.viewZenithCosine = clamp(viewDirection.y, -1.0, 1.0);
    ray.sunAzimuthCosine = azimuthCosine;
    return ray;
}

// 観測点から地表へ引いた接線の向きの天頂余弦。これより下を向く視線が地表と交わる。
float skyHorizonZenithCosine(float bottomRadius, float observerRadius)
{
    float ratio = bottomRadius / observerRadius;
    return -sqrt(max(0.0, 1.0 - ratio * ratio));
}

// スカイビューLUTの標本。単位位置をテクセル中心のUVへ直して引くため、両端のテクセルの条件も補間で覆える。
float3 sampleSkyView(AtmosphereMedium medium, float observerRadius, SkyViewRay ray)
{
    uint width;
    uint height;
    skyViewLut.GetDimensions(width, height);
    float2 unitPosition = skyViewUnitFromRay(medium, observerRadius, ray);
    return skyViewLut.SampleLevel(texelCenterUvFromUnit(unitPosition, uint2(width, height)), 0.0).rgb;
}

// 観測点からその画素の視線へ向かう透過率。太陽円盤の放射輝度を大気で減らすために引く。
// 円盤の中心方向でなく画素の視線を軸に取るのは、円盤が有限の角半径を持ち、地平線をまたぐ間は
// 画素ごとに経路が違うためである(正本は`crates/blitz_render/src/atmosphere/sun_visibility.rs`)。
// 注意: 惑星と交わる向きの値は地面を貫いた先の大気を含むため意味を持たない。交わる向きを外すのは呼び出し側である。
float3 sunTransmittanceAlongView(AtmosphereMedium medium, float observerRadius, float viewZenithCosine)
{
    AtmosphereRay ray;
    ray.radius = observerRadius;
    ray.zenithCosine = viewZenithCosine;
    return transmittanceLut.SampleLevel(transmittanceUvFromRay(medium, ray), 0.0).rgb;
}
