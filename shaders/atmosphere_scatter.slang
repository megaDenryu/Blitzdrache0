// 多重散乱の積分が使う補助: ある点の散乱係数と消散係数、球との最近交差、太陽への透過率と地表の遮蔽。
// 注意: これらの式は`crates/blitz_render/src/atmosphere`の写しである(medium_sample.rs・intersect.rs・
// multiscatter_step.rs)。正本はRust側にあり、単調性と値域のユニットテストもそちらが持つ。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_mapping;
import atmosphere_medium;

// 地表の遮蔽を判定するときに惑星の中心を上向きへずらす量。判定する点が地表そのものにあるとき、
// 自分の足元との交差を拾って常に影と答えることを避ける。
static const float shadowOffsetMeters = 10.0;

struct MediumSample
{
    float3 scattering;
    float3 extinction;
};

MediumSample sampleMedium(AtmosphereMedium medium, float3 position)
{
    float altitude = length(position) - bottomRadiusOf(medium);
    float rayleighDensity = componentDensity(medium, 0, altitude);
    float mieDensity = componentDensity(medium, 1, altitude);
    float absorptionDensity = componentDensity(medium, 2, altitude);

    MediumSample sample;
    sample.scattering = medium.scattering[0].xyz * rayleighDensity + medium.scattering[1].xyz * mieDensity;
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

// その位置から太陽へ向かう向きの透過率。表を引くだけであり、惑星による遮蔽はここでは見ない。
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

// その位置から太陽へ向かう視線が惑星に遮られるか。
bool sunBlockedByGround(AtmosphereMedium medium, float3 position, float3 sunDirection)
{
    float radius = length(position);
    if (radius <= 0.0)
    {
        return true;
    }
    float3 shifted = position - (position / radius) * shadowOffsetMeters;
    return nearestSphereHit(shifted, sunDirection, bottomRadiusOf(medium)) >= 0.0;
}
