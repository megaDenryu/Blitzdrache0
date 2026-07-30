// 多重散乱の積分が使う補助: ある点の散乱係数と消散係数、球との最近交差、太陽への透過率と地表の遮蔽。
// 注意: これらの式は`crates/blitz_render/src/atmosphere`の写しである(medium_sample.rs・intersect.rs・
// multiscatter_step.rs)。正本はRust側にあり、単調性と値域のユニットテストもそちらが持つ。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_mapping;
import atmosphere_medium;

// 地表の遮蔽を判定するときに遮蔽球の半径をこの量だけ縮める。判定する点が地表そのものにあるとき、
// 自分の足元との交差を拾って常に影と答えることを避ける。標本点を動かす形を採らない理由は正本にある。
// 正本は`crates/blitz_render/src/atmosphere/integration/sun_visibility.rs`の`遮蔽球を縮める半径メートル`であり、
// 両者の値が一致することは`cargo xtask conform`が機械検査する。
static const float shadowSphereShrinkMeters = 10.0;

// 空気分子と浮遊粒子の散乱を和のほかに別々にも持つのは、一次散乱の積分が2つへ別々の位相関数を掛けるためである。
// 多重散乱の近似は向きの偏りを失った光を扱うため和だけを使う。
struct MediumSample
{
    float3 scattering;
    float3 rayleighScattering;
    float3 mieScattering;
    float3 extinction;
};

MediumSample sampleMedium(AtmosphereMedium medium, float3 position)
{
    float altitude = length(position) - bottomRadiusOf(medium);
    float rayleighDensity = componentDensity(medium, 0, altitude);
    float mieDensity = componentDensity(medium, 1, altitude);
    float absorptionDensity = componentDensity(medium, 2, altitude);

    MediumSample sample;
    sample.rayleighScattering = medium.scattering[0].xyz * rayleighDensity;
    sample.mieScattering = medium.scattering[1].xyz * mieDensity;
    sample.scattering = sample.rayleighScattering + sample.mieScattering;
    sample.extinction = medium.extinction[0].xyz * rayleighDensity + medium.extinction[1].xyz * mieDensity
        + medium.extinction[2].xyz * absorptionDensity;
    return sample;
}

// 惑星中心を原点とする位置と単位ベクトルの向きから、半径sphereRadiusの球との最も近い交点までの距離を求める。
// 交わらないとき、および交点がすべて背後にあるときは負を返す。
float nearestSphereHit(float3 position, float3 direction, float sphereRadius)
{
    float b = 2.0 * dot(direction, position);
    float c = dot(position, position) - sphereRadius * sphereRadius;
    float discriminant = b * b - 4.0 * c;
    if (discriminant < 0.0)
    {
        return -1.0;
    }
    float root = sqrt(discriminant);
    float near = (-b - root) * 0.5;
    float far = (-b + root) * 0.5;
    if (far < 0.0)
    {
        return -1.0;
    }
    return near < 0.0 ? far : near;
}

// その位置から太陽へ向かう向きの透過率。表を参照するだけであり、惑星による遮蔽はここでは見ない。
float3 sunTransmittance(AtmosphereMedium medium, Sampler2D transmittanceLut, float3 position, float3 sunDirection)
{
    float radius = length(position);
    if (radius <= 0.0)
    {
        return float3(0.0, 0.0, 0.0);
    }
    AtmosphereRay ray;
    ray.radius = radius;
    ray.zenithCosine = clamp(dot(position / radius, sunDirection), -1.0, 1.0);
    return transmittanceLut.SampleLevel(transmittanceUvFromRay(medium, ray), 0.0).rgb;
}

// その位置から太陽へ向かう視線が惑星に遮られるか。縮めた球で判定するため、遮蔽が始まる太陽の高度が
// 真の地平線より約0.1度だけ低い方へずれる(根拠は正本の`太陽が地表に遮られるか`)。
bool sunBlockedByGround(AtmosphereMedium medium, float3 position, float3 sunDirection)
{
    return nearestSphereHit(position, sunDirection, bottomRadiusOf(medium) - shadowSphereShrinkMeters) >= 0.0;
}

// 視線が地表で終わるとき、地面が拡散反射で返す光。多重散乱の積分とスカイビューの積分が同じ式を使う。
// 遮蔽判定を通さずに透過率を参照するのは、地表そのものにある点が自分の足元と交わるためである(照り返しの
// 向きの妥当性は入射余弦が受け持つ)。正本は`crates/blitz_render/src/atmosphere/integration/multiscatter_step.rs`である。
float3 groundBounce(
    AtmosphereMedium medium, Sampler2D transmittanceLut, float3 groundPosition, float3 sunDirection, float3 throughput)
{
    float groundRadius = max(1.0, length(groundPosition));
    float3 upVector = groundPosition / groundRadius;
    float3 toSun = sunTransmittance(medium, transmittanceLut, groundPosition, sunDirection);
    float incidentCosine = clamp(dot(upVector, sunDirection), 0.0, 1.0);
    return toSun * throughput * incidentCosine * medium.groundAlbedoAndAsymmetry.xyz / atmospherePi;
}
