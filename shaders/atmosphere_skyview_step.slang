// 視線の上で1区間ぶんを積む操作。区間の中で消散が一定であるとみなし、閉形式で積む。
// 源の項は位相関数つきの一次散乱と、多重散乱LUTから引いた等方の項の和である。
// スカイビューLUTと空中遠近ボリュームの両方がこの1つの関数を呼ぶ。源の項が2つで違うと、地平の空と遠景の
// ジオメトリが同じ媒質に見えないためである。
// 注意: この式は`crates/blitz_render/src/atmosphere/integration/skyview_step.rs`の写しである。正本はRust側にある。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_medium;
import atmosphere_scatter;
import atmosphere_skyview_geometry;

void accumulateViewSegment(
    inout float3 luminance,
    inout float3 throughput,
    AtmosphereMedium medium,
    Sampler2D transmittanceLut,
    Sampler2D multiScatterLut,
    float3 position,
    float3 sunDirection,
    float segmentLength,
    float2 phase)
{
    MediumSample sample = sampleMedium(medium, position);
    float3 toSun = sunBlockedByGround(medium, position, sunDirection)
        ? float3(0.0, 0.0, 0.0)
        : sunTransmittance(medium, transmittanceLut, position, sunDirection);
    float3 multiScatter = sampleMultiScatter(medium, multiScatterLut, position, sunDirection);
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
