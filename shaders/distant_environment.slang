// 遠方環境の立方体画像の生成。テクセルが表す太陽相対座標の向きについて天空放射輝度供給を参照し、
// 太陽円盤を除いた連続な放射輝度を書き込む。書き込み先は立方体互換の2次元配列であり、奥行きが面の層番号である。
// 太陽相対座標で焼くため、太陽の方位はこのシェーダーへ1つも入らない。
// 注意: 数式の正本は`crates/blitz_render/src/atmosphere`(distant_environment_lut.rs・cube_face_mapping.rs・
// sun_relative_frame.rs・sky_radiance.rs)にあり、ここはその写しである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_medium;
import atmosphere_skyview_mapping;
import cube_face_mapping;
import sky_radiance_supply;
import skyview_ray_lookup;
import sun_relative_frame;

[[vk::binding(0, 0)]] ConstantBuffer<AtmosphereMedium> atmosphereMedium;
[[vk::binding(1, 0)]] Sampler2D skyViewLut;
[[vk::binding(2, 0)]] [format("rgba16f")] RWTexture2DArray<float4> distantEnvironment;

// そのフレームの観測点・太陽の高度・明るさの尺度。媒体のシェーダー定数と別に即時定数で渡すのは、
// 媒体が変わらないまま時刻や空描画方針だけが動くフレームでも、この4つが書き換わるためである。
struct DistantEnvironmentCondition
{
    float observerRadius;
    float sunZenithCosine;
    float sunIrradiance;
    float reserved;
    float4 nightSkyRadiance;
};
[[vk::push_constant]] DistantEnvironmentCondition distantEnvironmentCondition;

static const uint threadsPerSide = 8;

[shader("compute")]
[numthreads(threadsPerSide, threadsPerSide, 1)]
void computeMain(uint3 threadId : SV_DispatchThreadID)
{
    uint width;
    uint height;
    uint layers;
    distantEnvironment.GetDimensions(width, height, layers);
    if (threadId.x >= width || threadId.y >= height || threadId.z >= layers)
    {
        return;
    }

    float2 facePosition = cubeFaceTexelCenter(threadId.xy, width);
    float3 direction = directionOfCubeFace(threadId.z, facePosition);
    float3 sunDirection = sunRelativeSunDirection(distantEnvironmentCondition.sunZenithCosine);
    SkyViewRay ray = skyViewRayOfWorldDirection(direction, sunDirection);
    float3 radiance = skyRadianceSupply(
        atmosphereMedium,
        skyViewLut,
        distantEnvironmentCondition.observerRadius,
        ray,
        distantEnvironmentCondition.sunIrradiance,
        distantEnvironmentCondition.nightSkyRadiance.rgb);

    distantEnvironment[threadId] = float4(radiance, 0.0);
}
