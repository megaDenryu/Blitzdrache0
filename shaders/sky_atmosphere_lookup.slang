// 空パスが大気のベイク済み画像を参照する工程。受け取るのはワールドの視線と観測半径、返すのはスカイビューのベイク済み画像の放射輝度と
// 太陽方向の透過率である。参照するための資源(媒体のシェーダー定数と2枚のベイク済み画像)もここが宣言する。
// 注意: 写像の正本は`crates/blitz_render/src/atmosphere`(skyview_lookup.rs・skyview_mapping.rs・
// transmittance_mapping.rs・intersect.rs)にあり、ここはその写しである。
// ワールドの視線からベイク済み画像の視線を求める写しは`skyview_ray_lookup`が持つ(遠方環境生成と共有するため)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_mapping;
import atmosphere_medium;
import atmosphere_skyview_mapping;
__exported import skyview_ray_lookup;

// set1は空パスが大気のベイク済み画像を参照するためだけのディスクリプタである。媒体のシェーダー定数を持つのは、ベイク済み画像の写像が
// 惑星下端半径と大気上端半径を要るためである。2枚のベイク済み画像のレイアウトはGENERALであり、レンダーグラフの
// 画像用途「大気のベイク済み画像の画素段参照」が同じレイアウトを指す。
[[vk::binding(0, 1)]] ConstantBuffer<AtmosphereMedium> atmosphereMedium;
[[vk::binding(1, 1)]] Sampler2D transmittanceLut;
[[vk::binding(2, 1)]] Sampler2D skyViewLut;

// 観測点から地表へ引いた接線の向きの天頂余弦。これより下を向く視線が地表と交わる。
float skyHorizonZenithCosine(float bottomRadius, float observerRadius)
{
    float ratio = bottomRadius / observerRadius;
    return -sqrt(max(0.0, 1.0 - ratio * ratio));
}

// スカイビューのベイク済み画像の標本。単位位置をテクセル中心のUVへ直して参照するため、両端のテクセルの条件も補間で覆える。
float3 sampleSkyView(AtmosphereMedium medium, float observerRadius, SkyViewRay ray)
{
    uint width;
    uint height;
    skyViewLut.GetDimensions(width, height);
    float2 unitPosition = skyViewUnitFromRay(medium, observerRadius, ray);
    return skyViewLut.SampleLevel(texelCenterUvFromUnit(unitPosition, uint2(width, height)), 0.0).rgb;
}

// 観測点からその画素の視線へ向かう透過率。太陽円盤の放射輝度を大気で減らすために参照する。
// 円盤の中心方向でなく画素の視線を軸に取るのは、円盤が有限の角半径を持ち、地平線をまたぐ間は
// 画素ごとに経路が違うためである(正本は`crates/blitz_render/src/atmosphere/integration/sun_visibility.rs`)。
// 注意: 惑星と交わる向きの値は地面を貫いた先の大気を含むため意味を持たない。交わる向きを外すのは呼び出し側である。
float3 sunTransmittanceAlongView(AtmosphereMedium medium, float observerRadius, float viewZenithCosine)
{
    AtmosphereRay ray;
    ray.radius = observerRadius;
    ray.zenithCosine = viewZenithCosine;
    return transmittanceLut.SampleLevel(transmittanceUvFromRay(medium, ray), 0.0).rgb;
}
