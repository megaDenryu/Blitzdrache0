// 空中遠近ボリュームの生成。ボクセルが表す画面の位置とカメラからの距離について経路を積み、散乱をRGBへ、
// 代表透過率をAへ書く。太陽の放射照度は1として焼き、合成側が掛ける。
// 注意: 数式の正本は`crates/blitz_render/src/atmosphere`(bake/aerial_lut.rs・integration/aerial_march.rs・
// mapping/aerial_mapping.rs)にあり、ここはその写しである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_aerial_mapping;
import atmosphere_aerial_march;
import atmosphere_medium;
import atmosphere_skyview_geometry;

[[vk::binding(0, 0)]] ConstantBuffer<AtmosphereMedium> atmosphereMedium;
[[vk::binding(1, 0)]] Sampler2D transmittanceLut;
[[vk::binding(2, 0)]] Sampler2D multiScatterLut;
[[vk::binding(3, 0)]] [format("rgba16f")] RWTexture3D<float4> aerialVolume;

// そのフレームの観測点・太陽の向き・視錐台・最遠距離。媒体のユニフォームと別に押し込み定数で渡すのは、
// 媒体が変わらないままカメラと太陽だけが動くフレームでもこれらが毎回書き換わるためである。
struct AerialCondition
{
    float4 baseDirectionAndObserverRadius;
    float4 horizontalDeltaAndFarDistance;
    float4 verticalDelta;
    float4 sunDirection;
};
[[vk::push_constant]] AerialCondition aerialCondition;

static const uint threadsPerSide = 4;

[shader("compute")]
[numthreads(threadsPerSide, threadsPerSide, threadsPerSide)]
void computeMain(uint3 threadId : SV_DispatchThreadID)
{
    uint width;
    uint height;
    uint depth;
    aerialVolume.GetDimensions(width, height, depth);
    if (threadId.x >= width || threadId.y >= height || threadId.z >= depth)
    {
        return;
    }

    AerialFrustumRays rays;
    rays.baseDirection = aerialCondition.baseDirectionAndObserverRadius.xyz;
    rays.horizontalDelta = aerialCondition.horizontalDeltaAndFarDistance.xyz;
    rays.verticalDelta = aerialCondition.verticalDelta.xyz;

    SkyViewGeometry geometry;
    geometry.observerPosition = float3(0.0, aerialCondition.baseDirectionAndObserverRadius.w, 0.0);
    geometry.viewDirection = aerialViewDirection(rays, aerialScreenPositionOfVoxel(threadId.xy, width));
    geometry.sunDirection = aerialCondition.sunDirection.xyz;

    // 1区間の長さは解像度と最遠距離だけで決まる。奥行きの添字がそのまま積む区間の数の係数になる。
    float intervalCount = float(depth - 1) * float(aerialSegmentsPerSlice);
    float segmentLength = aerialCondition.horizontalDeltaAndFarDistance.w / intervalCount;
    AerialPathResult path = marchAerialPath(
        atmosphereMedium, transmittanceLut, multiScatterLut, geometry, threadId.z * aerialSegmentsPerSlice,
        segmentLength);

    aerialVolume[threadId] = float4(path.scattering, aerialRepresentativeTransmittance(path.throughput));
}
