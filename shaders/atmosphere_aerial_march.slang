// 空中遠近ボリュームの1ボクセルぶんの積分。カメラから与えられた区間数ぶんだけ視線を刻み、散乱と通過率を積む。
// 1区間ぶんの積み方はスカイビューと同じ`atmosphere_skyview_step`を使う。
// 区間の刻みを全ボクセルで同じ格子に固定し、ボクセルごとに区間数だけを変えるため、遠いスライスの積分が
// 近いスライスの積分の厳密な延長になる。経路を惑星の地表でも大気上端でも切らないのは、規格化密度がどちらの
// 外側でも0になり、切る場合と同じ値が出るためである。
// 注意: この積分は`crates/blitz_render/src/atmosphere/integration/aerial_march.rs`の写しである。正本はRust側にある。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_medium;
import atmosphere_skyview_geometry;
import atmosphere_skyview_step;

// 1つの距離スライスを何区間で刻むか。正本は`crates/blitz_render/src/atmosphere/integration/aerial_march.rs`の
// `スライスあたりの標本区間数`であり、ここはその写しである。両者の値が一致することは`cargo xtask conform`が機械検査する。
static const uint aerialSegmentsPerSlice = 2;
// 区間の中で媒体を評価する位置を、区間の始まりから区間長の何倍だけ進んだ所に置くか。積分幅ではない。
static const float aerialStepSamplePosition = 0.3;

// 経路が加えた散乱と、経路が残した通過率。
struct AerialPathResult
{
    float3 scattering;
    float3 throughput;
};

AerialPathResult marchAerialPath(
    AtmosphereMedium medium,
    Sampler2D transmittanceLut,
    Sampler2D multiScatterLut,
    SkyViewGeometry geometry,
    uint segmentCount,
    float segmentLength)
{
    AerialPathResult result;
    result.scattering = float3(0.0, 0.0, 0.0);
    result.throughput = float3(1.0, 1.0, 1.0);
    if (segmentCount == 0 || segmentLength <= 0.0)
    {
        return result;
    }

    float2 phase = skyViewPhasePair(medium, geometry);
    for (uint step = 0; step < segmentCount; ++step)
    {
        float sampleDistance = (float(step) + aerialStepSamplePosition) * segmentLength;
        float3 position = geometry.observerPosition + geometry.viewDirection * sampleDistance;
        accumulateViewSegment(
            result.scattering, result.throughput, medium, transmittanceLut, multiScatterLut, position,
            geometry.sunDirection, segmentLength, phase);
    }
    return result;
}

// 3成分の通過率を1つの代表値へ丸める。等重みの平均を採るのは、色に偏らずに面の色がどれだけ残るかを表すためである。
float aerialRepresentativeTransmittance(float3 throughput)
{
    return clamp((throughput.x + throughput.y + throughput.z) / 3.0, 0.0, 1.0);
}
