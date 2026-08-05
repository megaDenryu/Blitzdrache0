// 中央差分に使う隣の画素を軸ごとに1つ選ぶ工程。受け取るのは深度画像と射影と標点と同一面の許容と軸と画素の位置、
// 返すのは選んだ隣点への変位、または同じ面に載る隣点が無いという答えである。
// 注意: この選び方は`crates/blitz_render/src/local_visibility/neighbor_selection.rs`の写しである。正本はRust側にある。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import local_visibility_depth;
import local_visibility_projection;

// 隣点1つの候補。向きは常に画素の位置が増える側であり、負側の隣点では標点から見た向きを反転して作る。
struct LocalVisibilityNeighbor
{
    bool available;
    float3 displacement;
};

// 軸の片側1つの候補。画面の外と空の画素は候補にならない。
LocalVisibilityNeighbor localVisibilityNeighborCandidate(
    Texture2D<float> depthImage, LocalVisibilityProjection projection, uint2 extent,
    float3 origin, int2 neighborPixel, bool positiveSide)
{
    LocalVisibilityNeighbor candidate;
    candidate.available = false;
    candidate.displacement = float3(0.0, 0.0, 0.0);
    if (!localVisibilityContains(extent, neighborPixel))
    {
        return candidate;
    }
    uint2 pixel = uint2(uint(neighborPixel.x), uint(neighborPixel.y));
    if (!localVisibilityHasGeometry(depthImage, pixel))
    {
        return candidate;
    }
    float3 neighbor = localVisibilityRestoredPosition(projection, extent, pixel, localVisibilityDepthAt(depthImage, pixel));
    candidate.available = true;
    candidate.displacement = positiveSide ? neighbor - origin : origin - neighbor;
    return candidate;
}

// その軸で採る隣点への変位。両隣のうち復元した位置までの距離が小さい側を選ぶのは、物の輪郭では片側の隣点が
// 背景の遠い面に載るためである。常に片側だけを見ると、輪郭の内側1列の法線が背景へ向かって倒れる。
// 深度の値の差でなく復元した位置の距離で比べるのは、深度が奥行きに対して線形でないためである。
LocalVisibilityNeighbor localVisibilityAxisNeighbor(
    Texture2D<float> depthImage, LocalVisibilityProjection projection, uint2 extent,
    float3 origin, float sameSurfaceTolerance, int2 axisStep, uint2 pixel)
{
    int2 center = int2(int(pixel.x), int(pixel.y));
    LocalVisibilityNeighbor negativeSide =
        localVisibilityNeighborCandidate(depthImage, projection, extent, origin, center - axisStep, false);
    LocalVisibilityNeighbor positiveSide =
        localVisibilityNeighborCandidate(depthImage, projection, extent, origin, center + axisStep, true);

    LocalVisibilityNeighbor chosen;
    chosen.available = false;
    chosen.displacement = float3(0.0, 0.0, 0.0);
    if (negativeSide.available && positiveSide.available)
    {
        bool positiveIsNearer = length(positiveSide.displacement) < length(negativeSide.displacement);
        chosen = positiveIsNearer ? positiveSide : negativeSide;
    }
    else if (negativeSide.available)
    {
        chosen = negativeSide;
    }
    else if (positiveSide.available)
    {
        chosen = positiveSide;
    }
    else
    {
        return chosen;
    }

    float coverage = localVisibilityPixelCoverage(projection, extent, localVisibilityCameraDepth(origin));
    if (length(chosen.displacement) > coverage * sameSurfaceTolerance)
    {
        chosen.available = false;
    }
    return chosen;
}
