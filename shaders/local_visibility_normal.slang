// 深度だけからビュー空間の法線を組み立てる工程。受け取るのは深度画像と射影と同一面の許容と画素の位置、
// 返すのは再構成した法線、または面が定まらないという答えである。
// 面が定まらない画素は呼び出し元が遮蔽なしへ退避する。輪郭では両隣が別の面に載り、どちらを選んでも
// 実在しない面の法線が出るためである。
// 注意: この組み立ては`crates/blitz_render/src/local_visibility/normal_reconstruction.rs`と
// `view_direction.rs`の写しである。正本はRust側にある。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import local_visibility_depth;
import local_visibility_neighbor;
import local_visibility_projection;

// 長さが1に保たれた検証の許容。単精度の平方根と除算を通した正規化がこの幅に収まる。
static const float localVisibilityUnitLengthTolerance = 1.0e-3;

// 法線の再構成の結果。
struct LocalVisibilityNormal
{
    bool reconstructed;
    float3 direction;
};

// 2つの変位が張る面に垂直な単位向き。2つが平行で外積が潰れる場合は面が定まらない。
LocalVisibilityNormal localVisibilityCrossDirection(float3 first, float3 second)
{
    LocalVisibilityNormal result;
    result.reconstructed = false;
    result.direction = float3(0.0, 0.0, 0.0);
    float3 crossed = cross(first, second);
    float crossedLength = length(crossed);
    if (!isfinite(crossedLength) || crossedLength <= 0.0)
    {
        return result;
    }
    float3 normalized = crossed / crossedLength;
    if (abs(length(normalized) - 1.0) > localVisibilityUnitLengthTolerance)
    {
        return result;
    }
    result.reconstructed = true;
    result.direction = normalized;
    return result;
}

// 外積の向きは隣点を選んだ側で符号が入れ替わるため、カメラから標点へ向かう向きと逆を向く側へ揃える。
float3 localVisibilityFacingCamera(float3 direction, float3 origin)
{
    float originLength = length(origin);
    if (!isfinite(originLength) || originLength <= 0.0)
    {
        return direction;
    }
    float3 viewLine = origin / originLength;
    return dot(direction, viewLine) > 0.0 ? -direction : direction;
}

// 画素の法線。返す法線は必ずカメラの側を向く。
// 前提: 呼び出し元はその画素にジオメトリが描かれていることを確かめている。
LocalVisibilityNormal localVisibilityReconstructNormal(
    Texture2D<float> depthImage, LocalVisibilityProjection projection, uint2 extent,
    float sameSurfaceTolerance, uint2 pixel)
{
    LocalVisibilityNormal failure;
    failure.reconstructed = false;
    failure.direction = float3(0.0, 0.0, 0.0);

    float3 origin = localVisibilityRestoredPosition(projection, extent, pixel, localVisibilityDepthAt(depthImage, pixel));
    LocalVisibilityNeighbor horizontal = localVisibilityAxisNeighbor(
        depthImage, projection, extent, origin, sameSurfaceTolerance, int2(1, 0), pixel);
    if (!horizontal.available)
    {
        return failure;
    }
    LocalVisibilityNeighbor vertical = localVisibilityAxisNeighbor(
        depthImage, projection, extent, origin, sameSurfaceTolerance, int2(0, 1), pixel);
    if (!vertical.available)
    {
        return failure;
    }
    LocalVisibilityNormal crossed = localVisibilityCrossDirection(horizontal.displacement, vertical.displacement);
    if (!crossed.reconstructed)
    {
        return failure;
    }
    LocalVisibilityNormal result;
    result.reconstructed = true;
    result.direction = localVisibilityFacingCamera(crossed.direction, origin);
    return result;
}
