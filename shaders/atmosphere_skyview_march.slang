// スカイビューの視線1本ぶんの積分。経路を刻み、一次散乱に位相関数を掛けた項と多重散乱のベイク済み画像から参照した
// 等方の項を積む。1区間ぶんの積み方は`atmosphere_skyview_step`が持ち、ここは経路の刻みと終わりの決め方だけを持つ。
// 注意: この積分は`crates/blitz_render/src/atmosphere/integration/skyview_march.rs`と`skyview_step.rs`の写しである。
// 正本はRust側にあり、値域と単調性のユニットテストもそちらが持つ。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_march;
import atmosphere_medium;
import atmosphere_scatter;
import atmosphere_skyview_geometry;
import atmosphere_skyview_step;

// 視線の経路を刻む区間の数。正本は`crates/blitz_render/src/atmosphere/integration/skyview_march.rs`の`標本区間数`であり、
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
        accumulateViewSegment(
            luminance, throughput, medium, transmittanceLut, multiScatterLut, position, geometry.sunDirection,
            segmentLength, phase);
    }

    // 積分幅の総和が経路長に等しいため、ここへ渡るthroughputは観測点から地表までの全経路ぶんである。
    if (groundHit > 0.0 && pathDistance == groundHit)
    {
        float3 groundPosition = geometry.observerPosition + geometry.viewDirection * groundHit;
        luminance += groundBounce(medium, transmittanceLut, groundPosition, geometry.sunDirection, throughput);
    }
    return luminance;
}
