// 局所可視性補正の第二工程。生の局所可視度を、同じ面に載る画素の間だけで均すコンピュートである。
// 均しが要るのは、標本の方位を画素ごとに回してあるためである。回転で散らした偏りは1画素だけを見ると
// 雑音として残り、隣り合う画素をまとめて初めて打ち消し合う。
// 面をまたいで均さない判定に、標点の法線が張る平面からの距離を使う。奥行きの差だけで判定すると、
// 斜めから見た床が別の面と見なされ、床の陰りが均されないまま雑音を残す。
// 注意: この工程は`crates/blitz_render/src/local_visibility/bilateral_blur.rs`の写しである。正本はRust側にある。
// 注意: 生の可視度を記憶画像として読むのは、書いた直後の画像をレイアウトを往復させずに読むためである。
// 注意: 記憶画像の形式宣言は`crates/blitz_render/src/vulkan/local_visibility/images.rs`の`局所可視度の形式`
// (R8G8B8A8_UNORM)と一致させる。Vulkanは形式を宣言した記憶画像の読み書きに画像ビューと同じ形式を要求するため、
// 食い違うとパイプラインの生成が落ちる。使うのは第1成分だけであり、残りの3成分は書く値を持たない。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import local_visibility_depth;
import local_visibility_normal;
import local_visibility_projection;
import local_visibility_setting;

[[vk::binding(0, 0)]] Texture2D<float> depthImage;
[[vk::binding(1, 0)]] [format("rgba8")] RWTexture2D<float4> rawVisibilityImage;
[[vk::binding(2, 0)]] [format("rgba8")] RWTexture2D<float4> blurredVisibilityImage;
[[vk::push_constant]] LocalVisibilitySetting localVisibilitySetting;

// 1班が覆う画素の一辺。遮蔽の標本化と同じ値であり、CPU側の班数の割り出しはこの値と一致させる。
static const uint threadsPerSide = 8;

// 画素1つの均した可視度。深度が書かれていない画素は遮蔽なし、面が定まらない画素は生の値をそのまま返す。
float blurredLocalVisibility(uint2 pixel)
{
    LocalVisibilitySetting setting = localVisibilitySetting;
    int2 center = int2(int(pixel.x), int(pixel.y));
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
        return rawVisibilityImage[center].r;
    }
    float coverage = localVisibilityPixelCoverage(setting.projection, setting.extent, localVisibilityCameraDepth(origin));
    float limit = localVisibilitySameSurfaceLimit(coverage, setting.blurSameSurfaceTolerance);
    int radius = int(setting.blurPixelRadius);
    float sum = 0.0;
    uint count = 0;
    for (int verticalOffset = -radius; verticalOffset <= radius; verticalOffset++)
    {
        for (int horizontalOffset = -radius; horizontalOffset <= radius; horizontalOffset++)
        {
            int2 neighborPixel = center + int2(horizontalOffset, verticalOffset);
            if (!localVisibilityContains(setting.extent, neighborPixel))
            {
                continue;
            }
            uint2 neighbor = uint2(uint(neighborPixel.x), uint(neighborPixel.y));
            if (!localVisibilityHasGeometry(depthImage, neighbor))
            {
                continue;
            }
            float3 neighborPosition = localVisibilityRestoredPosition(
                setting.projection, setting.extent, neighbor, localVisibilityDepthAt(depthImage, neighbor));
            if (abs(dot(neighborPosition - origin, normal.direction)) > limit)
            {
                continue;
            }
            sum += rawVisibilityImage[neighborPixel].r;
            count++;
        }
    }
    // 中心の画素は必ず窓を通るため件数は1以上になる。
    return saturate(sum / float(count));
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
    blurredVisibilityImage[int2(int(pixel.x), int(pixel.y))] = float4(blurredLocalVisibility(pixel), 0.0, 0.0, 1.0);
}
