// セルが占めるビュー空間の錐台と、そこへ余裕を掛ける工程。受け取るのはセルの3軸の添字と写像のパラメータ、
// 返すのは判定に使う錐台である。束縛先を1つも読まない。
// 注意: この形は`crates/blitz_render/src/clustered_lighting/cell_frustum.rs`と`cell_correspondence.rs`の写しである。
// 6つの平面でなく奥行きの区間と傾きの区間で持つのは、側面の4つが原点を通り`成分 = 傾き × 奥行き`という
// 同じ形をしているためである。
// 余裕を掛けるのは、狭いままの錐台が光の届く画素で光を落とし絵の欠損になるのに対し、広げた錐台は光を1件
// 余計に評価する費用にしかならないためである
// (正本と理由は`crates/blitz_render/src/clustered_lighting/cell_boundary_margin.rs`)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import cluster_mapping;

// 奥行きの区間を上下へ広げる相対量。CPU正本の`セルの奥行きを広げる相対量`と同じ値でなければならない。
static const float clusterDepthMarginRatio = 1.0e-5;
// 横と縦の区間を広げる画素数。CPU正本の`セルの横と縦を広げる画素数`と同じ値でなければならない。
static const float clusterTileMarginPixels = 1.0;

// 判定に使うセルの錐台。奥行きの区間と、横と縦それぞれの傾きの区間で表す。
struct ClusterCellFrustum
{
    float2 depthRange;
    float2 horizontalSlopeRange;
    float2 verticalSlopeRange;
};

// 段が占める奥行きの区間へ余裕を掛けた区間。手前の端の段は0まで、奥の端の段は無限まで伸ばす。
// 端を伸ばすのは、近クリップの手前と遠クリップの奥に表面が無く、そこで区間を切っても得るものが無いためである。
float2 clusterMarginedDepthRange(uint slice, float nearDepth, float farDepth, uint divisions)
{
    float lower = slice == 0 ? 0.0
                             : clusterSliceBoundViewDepth(slice, nearDepth, farDepth, divisions) * (1.0 - clusterDepthMarginRatio);
    float upper = slice + 1 >= divisions
                      ? 1.0e30
                      : clusterSliceBoundViewDepth(slice + 1, nearDepth, farDepth, divisions) * (1.0 + clusterDepthMarginRatio);
    return float2(lower, upper);
}

// タイルが占める正規化装置座標の区間へ、画素の余裕を掛けた区間。
float2 clusterMarginedTileNdcRange(uint index, uint divisions, uint extent)
{
    float2 range = clusterTileNdcRange(index, divisions);
    float margin = clusterTileMarginPixels * 2.0 / float(extent);
    return float2(range.x - margin, range.y + margin);
}

// セルが占める錐台。正規化装置座標の縦は下が正でビュー空間の縦は上が正であるため、縦の傾きは符号が反転して
// 上下の順序も入れ替わる。
ClusterCellFrustum clusterCellFrustumOf(
    uint3 cell, uint3 divisions, uint2 extent, float2 depthRange, float2 halfAngleTangent)
{
    float2 horizontalNdc = clusterMarginedTileNdcRange(cell.x, divisions.x, extent.x);
    float2 verticalNdc = clusterMarginedTileNdcRange(cell.y, divisions.y, extent.y);
    ClusterCellFrustum frustum;
    frustum.depthRange = clusterMarginedDepthRange(cell.z, depthRange.x, depthRange.y, divisions.z);
    frustum.horizontalSlopeRange = horizontalNdc * halfAngleTangent.x;
    frustum.verticalSlopeRange = float2(-verticalNdc.y, -verticalNdc.x) * halfAngleTangent.y;
    return frustum;
}
