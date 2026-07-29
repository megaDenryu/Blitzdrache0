// スカイビューの視線1本ぶんの積分。経路を刻み、一次散乱に位相関数を掛けた項と多重散乱LUTから引いた
// 等方の項を積む。区間の中で消散が一定であるとみなし、閉形式で積む(多重散乱の区間と同じ扱い)。
// 注意: この積分は`crates/blitz_render/src/atmosphere/skyview_march.rs`と`skyview_step.rs`の写しである。
// 正本はRust側にあり、値域と単調性のユニットテストもそちらが持つ。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_march;
import atmosphere_medium;
import atmosphere_scatter;
import atmosphere_skyview_geometry;

// 視線の経路を刻む区間の数。正本は`crates/blitz_render/src/atmosphere/skyview_march.rs`の`標本区間数`であり、
// ここはその写しである。両者の値が一致することは`cargo xtask conform`が機械検査する。
static const uint skyViewStepCount = 30;
// 区間の中で媒体を評価する位置を、区間の始まりから区間長の何倍だけ進んだ所に置くか。積分幅ではない。
static const float skyViewStepSamplePosition = 0.3;

float3 marchSkyView(
    AtmosphereMedium medium, Sampler2D transmittanceLut, Sampler2D multiScatterLut, SkyViewGeometry geometry)
{
    float3 luminance = float3(0.0, 0.0, 0.0);
    float groundHit = nearestSphereHit(geometry.observerPosition, geometry.viewDirection, bottomRadiusOf(medium));
    float topHit = nearestSphereHit(geometry.observerPosition, geometry.viewDirection, topRadiusOf(medium));
    float pathDistance = pathLength(groundHit, topHit);
    if (pathDistance <= 0.0)
    {
        return luminance;
    }

    float2 phase = skyViewPhasePair(medium, geometry);
    float segmentLength = pathDistance / float(skyViewStepCount);
    float3 throughput = float3(1.0, 1.0, 1.0);
    for (uint step = 0; step < skyViewStepCount; ++step)
    {
        float sampleDistance = (float(step) + skyViewStepSamplePosition) * segmentLength;
        float3 position = geometry.observerPosition + geometry.viewDirection * sampleDistance;
        MediumSample sample = sampleMedium(medium, position);
        float3 toSun = sunBlockedByGround(medium, position, geometry.sunDirection)
            ? float3(0.0, 0.0, 0.0)
            : sunTransmittance(medium, transmittanceLut, position, geometry.sunDirection);
        float3 multiScatter = sampleMultiScatter(medium, multiScatterLut, position, geometry.sunDirection);
        float3 segmentTransmittance = exp(-sample.extinction * segmentLength);
        for (uint channel = 0; channel < 3; ++channel)
        {
            float extinction = sample.extinction[channel];
            if (extinction > 0.0)
            {
                float single = toSun[channel]
                    * (sample.rayleighScattering[channel] * phase.x + sample.mieScattering[channel] * phase.y);
                float source = single + multiScatter[channel] * sample.scattering[channel];
                luminance[channel] += throughput[channel] * (source - source * segmentTransmittance[channel]) / extinction;
            }
            throughput[channel] *= segmentTransmittance[channel];
        }
    }

    // 積分幅の総和が経路長に等しいため、ここへ渡るthroughputは観測点から地表までの全経路ぶんである。
    if (groundHit > 0.0 && pathDistance == groundHit)
    {
        float3 groundPosition = geometry.observerPosition + geometry.viewDirection * groundHit;
        luminance += groundBounce(medium, transmittanceLut, groundPosition, geometry.sunDirection, throughput);
    }
    return luminance;
}
