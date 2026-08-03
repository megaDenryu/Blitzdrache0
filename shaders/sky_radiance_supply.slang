// 天空放射輝度供給: 太陽円盤を除いた連続な天空放射輝度を、視線1本について返す。
// 太陽円盤を含めないのは、円盤が直接光の経路として別に数えられるためである。含めると同じ光路を
// 空背景の視覚項と方向光の直接光項で二重に計上する。夜間背景放射輝度は昼夜の切替なしに常時足す。
// 注意: この式は`crates/blitz_render/src/atmosphere/supply/sky_radiance.rs`の写しである。正本はRust側にある。
// 注意: スカイビューのベイク済み画像を引数で受け取るのは、参照する側のディスクリプタの番号が消費者ごとに違うためである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_medium;
import atmosphere_skyview_mapping;

// 空の表を参照し、太陽の放射照度を掛けて夜間背景を足す。0で下から切るのは、双一次補間と半精度の丸めが
// 0の近くでわずかな負を作りうるためであり、放射輝度が負になることに物理的な意味が無いからである。
float3 skyRadianceSupply(
    AtmosphereMedium medium,
    Sampler2D skyViewLut,
    float observerRadius,
    SkyViewRay ray,
    float sunIrradiance,
    float3 nightSkyRadiance)
{
    uint width;
    uint height;
    skyViewLut.GetDimensions(width, height);
    float2 unitPosition = skyViewUnitFromRay(medium, observerRadius, ray);
    float3 sample = skyViewLut.SampleLevel(texelCenterUvFromUnit(unitPosition, uint2(width, height)), 0.0).rgb;
    return max(sample * sunIrradiance + nightSkyRadiance, 0.0);
}
