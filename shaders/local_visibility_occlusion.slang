// 局所可視性補正の第一工程。深度画像1枚から、ぼかす前の局所可視度を1画素ずつ書き出すコンピュートである。
// 標本を半球の面の上に置き、その点が深度画像の中で手前の面に隠れているかを数える。
// 深度が書かれていない画素と、法線の面が定まらない画素は遮蔽なしへ退避する。空の背景に陰りを描かないこと
// そのものがこの退避である。
// 注意: この工程は`crates/blitz_render/src/local_visibility/occlusion_sampling.rs`の写しである。正本はRust側にある。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import local_visibility_depth;
import local_visibility_normal;
import local_visibility_projection;
import local_visibility_rotation;
import local_visibility_sample;
import local_visibility_setting;

[[vk::binding(0, 0)]] Texture2D<float> depthImage;
[[vk::binding(1, 0)]] RWTexture2D<float> rawVisibilityImage;
[[vk::push_constant]] LocalVisibilitySetting localVisibilitySetting;

// 1班が覆う画素の一辺。CPU側の班数の割り出しはこの値と一致させる。
static const uint threadsPerSide = 8;

// 標本1本が遮蔽へ寄与する重み。遮蔽していない標本と、画面の外や空へ落ちた標本は0を返す。
float sampleOcclusionWeight(float3 origin, float3 normal, LocalVisibilityRotation rotation, float depthBias, uint index)
{
    LocalVisibilitySetting setting = localVisibilitySetting;
    float3 direction = localVisibilityHemisphereDirection(index, setting.sampleCount, rotation, normal);
    float3 samplePoint = origin + direction * setting.occlusionRadius;
    uint2 samplePixel;
    if (!localVisibilityPixelOfPosition(setting.projection, setting.extent, samplePoint, samplePixel))
    {
        return 0.0;
    }
    if (!localVisibilityHasGeometry(depthImage, samplePixel))
    {
        return 0.0;
    }
    float3 occluder = localVisibilityRestoredPosition(
        setting.projection, setting.extent, samplePixel, localVisibilityDepthAt(depthImage, samplePixel));
    float occluderDepth = localVisibilityCameraDepth(occluder);
    if (occluderDepth + depthBias >= localVisibilityCameraDepth(samplePoint))
    {
        return 0.0;
    }
    return localVisibilityFalloffWeight(setting, occluderDepth - localVisibilityCameraDepth(origin));
}

// 画素1つの局所可視度。
float pixelLocalVisibility(uint2 pixel)
{
    LocalVisibilitySetting setting = localVisibilitySetting;
    if (!localVisibilityHasGeometry(depthImage, pixel))
    {
        return 1.0;
    }
    float3 origin = localVisibilityRestoredPosition(
        setting.projection, setting.extent, pixel, localVisibilityDepthAt(depthImage, pixel));
    LocalVisibilityNormal normal = localVisibilityReconstructNormal(
        depthImage, setting.projection, setting.extent, setting.occlusionSameSurfaceTolerance, pixel);
    if (!normal.reconstructed)
    {
        return 1.0;
    }
    float coverage = localVisibilityPixelCoverage(setting.projection, setting.extent, localVisibilityCameraDepth(origin));
    float depthBias = localVisibilityDepthBias(coverage, setting.depthBiasScale);
    LocalVisibilityRotation rotation = localVisibilityRotationOfPixel(pixel);
    float weightSum = 0.0;
    for (uint index = 0; index < setting.sampleCount; index++)
    {
        weightSum += sampleOcclusionWeight(origin, normal.direction, rotation, depthBias, index);
    }
    return saturate(1.0 - weightSum * localVisibilitySampleWeight(setting.sampleCount));
}

[shader("compute")]
[numthreads(threadsPerSide, threadsPerSide, 1)]
void computeMain(uint3 threadId : SV_DispatchThreadID)
{
    uint2 pixel = threadId.xy;
    if (pixel.x >= localVisibilitySetting.extent.x || pixel.y >= localVisibilitySetting.extent.y)
    {
        return;
    }
    rawVisibilityImage[int2(int(pixel.x), int(pixel.y))] = pixelLocalVisibility(pixel);
}
