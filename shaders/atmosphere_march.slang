// 1本の向きについて経路を刻み、そこで散乱した光と「1回の散乱を経ても残る割合」を積む。
// 位相関数を使わず等方(1/4π)で扱うのがHillaire 2020の多重散乱近似である。
// 注意: この積分は`crates/blitz_render/src/atmosphere/multiscatter_march.rs`と`multiscatter_step.rs`の写しである。
// 球面の向きの並びは`sphere_directions.rs`の写しである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_medium;
import atmosphere_scatter;

static const float atmospherePi = 3.14159265358979323846;
static const uint sphereDirectionSide = 8;
static const uint sphereDirectionCount = sphereDirectionSide * sphereDirectionSide;
static const uint marchStepCount = 20;
// 区間の中で媒体を評価する位置を、区間の始まりから区間長の何倍だけ進んだ所に置くか。積分幅ではない。
static const float stepSamplePosition = 0.3;

struct MarchResult
{
    float3 luminance;
    float3 survival;
};

// 添字から向きの単位ベクトルを求める。天頂は+Zであり、方位角を等間隔に、天頂角の余弦を等間隔に刻む。
float3 sphereDirection(uint index)
{
    float azimuthSide = 0.5 + float(index / sphereDirectionSide);
    float zenithSide = 0.5 + float(index % sphereDirectionSide);
    float side = float(sphereDirectionSide);
    float azimuth = 2.0 * atmospherePi * (azimuthSide / side);
    float zenith = acos(clamp(1.0 - 2.0 * (zenithSide / side), -1.0, 1.0));
    float lateral = sin(zenith);
    return float3(cos(azimuth) * lateral, sin(azimuth) * lateral, cos(zenith));
}

// 地表と大気上端のどちらへ先に当たるかで経路の終わりを決める。交差の無い向きは負で表され、経路長0になる。
float pathLength(float groundHit, float topHit)
{
    if (groundHit < 0.0)
    {
        return max(0.0, topHit);
    }
    if (topHit < 0.0)
    {
        return groundHit;
    }
    return min(groundHit, topHit);
}

MarchResult marchDirection(
    AtmosphereMedium medium, Sampler2D transmittanceLut, float3 observerPosition, float3 viewDirection, float3 sunDirection)
{
    MarchResult result;
    result.luminance = float3(0.0, 0.0, 0.0);
    result.survival = float3(0.0, 0.0, 0.0);

    float groundHit = nearestSphereHit(observerPosition, viewDirection, bottomRadiusOf(medium));
    float pathDistance = pathLength(groundHit, nearestSphereHit(observerPosition, viewDirection, topRadiusOf(medium)));
    if (pathDistance <= 0.0)
    {
        return result;
    }

    float3 throughput = float3(1.0, 1.0, 1.0);
    float segmentLength = pathDistance / float(marchStepCount);
    float isotropicPhase = 1.0 / (4.0 * atmospherePi);
    for (uint step = 0; step < marchStepCount; ++step)
    {
        float sampleDistance = (float(step) + stepSamplePosition) * segmentLength;
        float3 position = observerPosition + viewDirection * sampleDistance;
        MediumSample sample = sampleMedium(medium, position);
        float3 toSun = sunBlockedByGround(medium, position, sunDirection)
            ? float3(0.0, 0.0, 0.0)
            : sunTransmittance(medium, transmittanceLut, position, sunDirection);
        float3 segmentTransmittance = exp(-sample.extinction * segmentLength);
        for (uint channel = 0; channel < 3; ++channel)
        {
            float extinction = sample.extinction[channel];
            if (extinction > 0.0)
            {
                float source = toSun[channel] * sample.scattering[channel] * isotropicPhase;
                result.luminance[channel] += throughput[channel] * (source - source * segmentTransmittance[channel]) / extinction;
                float remaining = sample.scattering[channel];
                result.survival[channel] += throughput[channel] * (remaining - remaining * segmentTransmittance[channel]) / extinction;
            }
            throughput[channel] *= segmentTransmittance[channel];
        }
    }

    // 積分幅の総和が経路長に等しいため、ここへ渡るthroughputは観測点から地表までの全経路ぶんである。
    if (groundHit > 0.0 && pathDistance == groundHit)
    {
        float3 groundPosition = observerPosition + viewDirection * groundHit;
        float groundRadius = max(1.0, length(groundPosition));
        float3 upVector = groundPosition / groundRadius;
        float3 toSun = sunTransmittance(medium, transmittanceLut, groundPosition, sunDirection);
        float incidentCosine = clamp(dot(upVector, sunDirection), 0.0, 1.0);
        result.luminance += toSun * throughput * incidentCosine * medium.groundAlbedoAndAsymmetry.xyz / atmospherePi;
    }
    return result;
}
