// 画素の位置と深度からビュー空間の量を復元する規約。透視投影の4つの量だけを持ち、逆行列を持たない。
// 注意: この式は`crates/blitz_render/src/local_visibility/projection.rs`と`view_position.rs`の写しである。正本はRust側にある。
// 注意: 正本は正規化装置座標と画素位置の組み立てを倍精度で行い、最後に単精度へ狭める。シェーダーは単精度だけで
// 組むため、最下位のビットが正本と食い違いうる。検収が全画素のバイト一致でなく符号値1段の許容で突き合わせるのは
// この差のためである(`参照: _doc/設計/放射輝度問い合わせ階層.md`「IIaの実装設計」)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 透視投影の逆変換に要る4つの量。深度の規約はVulkanのクリップ空間であり、0が近クリップ面、1が遠クリップ面である。
struct LocalVisibilityProjection
{
    float nearClip;
    float farClip;
    float halfFovTangent;
    float aspectRatio;
};

// 深度が表すカメラからの奥行き。
float localVisibilityViewDepthOfDepth(LocalVisibilityProjection projection, float depth)
{
    return projection.farClip * projection.nearClip / (projection.farClip - depth * (projection.farClip - projection.nearClip));
}

// カメラから見た奥行き。ビュー空間では視線が奥行きの負の向きであるため、符号を反転した値が距離になる。
float localVisibilityCameraDepth(float3 viewPosition)
{
    return -viewPosition.z;
}

// 画素の中心が正規化装置座標のどこに当たるか。横は右が正、縦は下が正である。
float2 localVisibilityPixelCenterNdc(uint2 extent, uint2 pixel)
{
    return float2(
        2.0 * (float(pixel.x) + 0.5) / float(extent.x) - 1.0,
        2.0 * (float(pixel.y) + 0.5) / float(extent.y) - 1.0);
}

// 画素の中心と深度が指すビュー空間の位置。
// 前提: 呼び出し元は画素が寸法の内側にあることを確かめている。
float3 localVisibilityRestoredPosition(LocalVisibilityProjection projection, uint2 extent, uint2 pixel, float depth)
{
    float viewDepth = localVisibilityViewDepthOfDepth(projection, depth);
    float2 ndc = localVisibilityPixelCenterNdc(extent, pixel);
    return float3(
        ndc.x * projection.halfFovTangent * projection.aspectRatio * viewDepth,
        -ndc.y * projection.halfFovTangent * viewDepth,
        -viewDepth);
}

// その奥行きで1画素が覆うビュー空間の幅。縦と横で違う場合は広い側を採る。
float localVisibilityPixelCoverage(LocalVisibilityProjection projection, uint2 extent, float viewDepth)
{
    float verticalWidth = 2.0 * projection.halfFovTangent * viewDepth / float(extent.y);
    float horizontalWidth = verticalWidth * projection.aspectRatio * float(extent.y) / float(extent.x);
    return max(verticalWidth, horizontalWidth);
}

// 実数の位置以下で最大の添字。非数と負は0へ、上限より大きい位置は上限へ張り付ける。
// 正本の`crates/blitz_render/src/numeric/floor_index.rs`が二分探索なのは、Rustに安全な浮動小数から整数への
// 変換が無いためである。シェーダーには床関数と整数への変換があるため、同じ規則を直に書く。
uint localVisibilityFloorIndex(float realPosition, uint maxIndex)
{
    if (isnan(realPosition) || realPosition <= 0.0)
    {
        return 0;
    }
    if (realPosition >= float(maxIndex))
    {
        return maxIndex;
    }
    return min(uint(floor(realPosition)), maxIndex);
}

// その位置が落ちる画素。近クリップ面より手前にある位置と、画面の外へ落ちた位置は偽を返す。
// 範囲へ張り付けて読まないのは、画面端の画素が端の1列を何度も遮蔽物として数え、画面の縁に暗い帯が出るためである。
bool localVisibilityPixelOfPosition(
    LocalVisibilityProjection projection, uint2 extent, float3 viewPosition, out uint2 pixel)
{
    pixel = uint2(0, 0);
    float viewDepth = localVisibilityCameraDepth(viewPosition);
    if (!isfinite(viewDepth) || viewDepth < projection.nearClip)
    {
        return false;
    }
    float ndcX = viewPosition.x / (projection.halfFovTangent * projection.aspectRatio * viewDepth);
    float ndcY = -viewPosition.y / (projection.halfFovTangent * viewDepth);
    float realX = (ndcX + 1.0) * 0.5 * float(extent.x);
    float realY = (ndcY + 1.0) * 0.5 * float(extent.y);
    if (!isfinite(realX) || realX < 0.0 || realX >= float(extent.x) || !isfinite(realY) || realY < 0.0 || realY >= float(extent.y))
    {
        return false;
    }
    pixel = uint2(localVisibilityFloorIndex(realX, extent.x - 1), localVisibilityFloorIndex(realY, extent.y - 1));
    return true;
}
