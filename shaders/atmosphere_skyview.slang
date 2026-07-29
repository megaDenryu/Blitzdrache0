// スカイビューLUTの生成。テクセルが表す視線について経路を刻み、一次散乱に位相関数を掛けた項と、
// 多重散乱LUTから引いた等方の項を足し込む。太陽の放射照度は1として焼き、描画側が掛ける。
// 注意: 数式の正本は`crates/blitz_render/src/atmosphere`(skyview_lut.rs・skyview_march.rs・skyview_step.rs・
// skyview_mapping.rs)にあり、ここはその写しである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_medium;
import atmosphere_skyview_geometry;
import atmosphere_skyview_march;
import atmosphere_skyview_mapping;

[[vk::binding(0, 0)]] ConstantBuffer<AtmosphereMedium> atmosphereMedium;
[[vk::binding(1, 0)]] Sampler2D transmittanceLut;
[[vk::binding(2, 0)]] Sampler2D multiScatterLut;
[[vk::binding(3, 0)]] [format("rgba16f")] RWTexture2D<float4> skyViewLut;

// そのフレームの観測点と太陽の向き。媒体のユニフォームと別に押し込み定数で渡すのは、媒体が変わらないまま
// 太陽だけが動くフレームでも、この2つだけは毎回書き換わるためである。
struct SkyViewCondition
{
    float observerRadius;
    float sunZenithCosine;
};
[[vk::push_constant]] SkyViewCondition skyViewCondition;

static const uint threadsPerSide = 8;

[shader("compute")]
[numthreads(threadsPerSide, threadsPerSide, 1)]
void computeMain(uint3 threadId : SV_DispatchThreadID)
{
    uint width;
    uint height;
    skyViewLut.GetDimensions(width, height);
    if (threadId.x >= width || threadId.y >= height)
    {
        return;
    }

    float radius = skyViewCondition.observerRadius;
    float2 unitPosition = endToEndUnitPosition(threadId.xy, uint2(width, height));
    SkyViewRay ray = rayFromSkyViewUnit(atmosphereMedium, radius, unitPosition);

    SkyViewGeometry geometry = skyViewGeometryOf(radius, skyViewCondition.sunZenithCosine, ray);
    float3 luminance = marchSkyView(atmosphereMedium, transmittanceLut, multiScatterLut, geometry);
    skyViewLut[threadId.xy] = float4(luminance, 0.0);
}
