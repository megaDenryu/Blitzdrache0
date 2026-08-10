// 局所光の影響範囲の球がセルの錐台と交わりうるかの判定。受け取るのは錐台と球、返すのは交わりうるかである。
// 注意: この判定は`crates/blitz_render/src/clustered_lighting/sphere_intersection.rs`の写しである。
// 判定は錐台を作る6つの半空間それぞれについて「球の中心から面までの符号付き距離が半径以内か」を見る形である。
// この形は偽陰性を出さない。交わる球は6つの半空間すべてから半径以内にあるためである。
// 注意: 交わらない球を交わると答えることはある。錐台の稜と角の近くで、6つの面すべてから半径以内でありながら
// 錐台とは交わらない位置が存在するためである。この偽陽性はそのセルで光を1件余計に評価する費用として現れ、
// 光の抜けにはならない。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import cluster_cell_frustum;

// 1つの軸が作る2つの側面までの符号付き距離を見る。側面は原点を通り`成分 = 傾き × 奥行き`で表されるため、
// 面の法線の長さは`sqrt(1 + 傾きの二乗)`であり、これで割ると符号付き距離になる。
bool clusterSideFacesWithinRadius(float component, float zComponent, float2 slopeRange, float radius)
{
    float lowerDistance = (-slopeRange.x * zComponent - component) / sqrt(1.0 + slopeRange.x * slopeRange.x);
    float upperDistance = (component + slopeRange.y * zComponent) / sqrt(1.0 + slopeRange.y * slopeRange.y);
    return lowerDistance <= radius && upperDistance <= radius;
}

// ビュー空間の球がこの錐台と交わりうるか。
bool clusterFrustumIntersectsSphere(ClusterCellFrustum frustum, float3 center, float radius)
{
    float viewDepth = -center.z;
    if (frustum.depthRange.x - viewDepth > radius || viewDepth - frustum.depthRange.y > radius)
    {
        return false;
    }
    return clusterSideFacesWithinRadius(center.x, center.z, frustum.horizontalSlopeRange, radius)
        && clusterSideFacesWithinRadius(center.y, center.z, frustum.verticalSlopeRange, radius);
}
