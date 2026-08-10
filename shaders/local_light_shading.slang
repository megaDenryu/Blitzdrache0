// 局所光の標本の列挙。受け取るのは画面の画素とビュー空間の奥行きと表面のカメラ相対位置、返すのはその画素へ
// 届く光の向きと放射である。材質の反射の式を1つも持たないため、PBRのシーンも両面ランバートの布も同じこの
// 1つを通して同じ並びの同じ光を読む(参照: `_doc/設計/放射輝度問い合わせ階層.md`「A. 直接光標本列挙」)。
//
// 消費側がクラスタ格子の作られ方を知らないのは、ここが区間の引き方を閉じているからである。消費側が知るのは
// 「この画素には何件の光が届き、n件目はどの向きから何を届けるか」だけである。
//
// 放射に減衰の窓を掛けるのは、クラスタの打ち切りと絵を一致させるためである。逆二乗のままでは影響半径の外側でも
// 寄与が0にならず、セルへ割り当てられた光と割り当てられなかった光の境目に段差が出る
// (正本は`crates/blitz_render/src/clustered_lighting/attenuation_window.rs`と`point_light_radiance.rs`)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import cluster_mapping;
import lighting_query;

// 距離の二乗へ足す微小量。CPU正本の`距離の二乗へ足す微小量`と同じ値でなければならない。
static const float localLightSquaredDistanceEpsilon = 1e-8;

// 1件の局所光がその表面へ届けるもの。向きは表面から光へ向かう単位ベクトルである。
struct LocalLightSample
{
    float3 directionToLight;
    float3 radiance;
};

// その画素が読むセルの区間。画面の外の画素も範囲の外の奥行きも端のセルへ入るため、どの画素も1つの区間を得る。
ClusterCell localLightClusterCellOfPixel(uint2 pixel, float viewDepth)
{
    uint3 divisions = lightingQueryHeader.clusterGridDivisions.xyz;
    uint2 extent = lightingQueryHeader.clusterScreenPixelExtent.xy;
    float2 depthRange = lightingQueryHeader.clusterDepthRangeAndHalfAngleTangent.xy;
    uint3 cell = uint3(
        clusterTileIndexOfPixel(pixel.x, extent.x, divisions.x),
        clusterTileIndexOfPixel(pixel.y, extent.y, divisions.y),
        clusterDepthSliceOfViewDepth(viewDepth, depthRange.x, depthRange.y, divisions.z));
    return clusterCells[clusterFlatCellIndex(cell, divisions)];
}

// その区間のn件目の光のレコード。
LocalLightRecord localLightRecordOfClusterSlot(ClusterCell cell, uint slot)
{
    return localLightRecords[clusterLightIndices[cell.start + slot]];
}

// レコードと表面の位置から、その表面へ届く向きと放射を求める。点光でない光種は偽を返して読み飛ばさせる。
// 光種の枝を明示するのは、種別を読まずに点光の式を当てると、光種が増えたときに新しい種別の光を黙って
// 点光の減衰で照らしてしまうためである。
bool localLightSampleOf(LocalLightRecord record, float3 relativePosition, out LocalLightSample result)
{
    result.directionToLight = float3(0.0, 0.0, 0.0);
    result.radiance = float3(0.0, 0.0, 0.0);
    if (record.kind.x != localLightKindPoint)
    {
        return false;
    }
    float3 toLight = record.cameraRelativePosition.xyz - relativePosition;
    float squaredDistance = dot(toLight, toLight);
    float lightDistance = sqrt(squaredDistance);
    float ratio = lightDistance / record.influenceRadius;
    float squaredRatio = ratio * ratio;
    float inner = clamp(1.0 - squaredRatio * squaredRatio, 0.0, 1.0);
    float window = inner * inner;
    // 光の位置にちょうど載った表面では向きが定まらないため零ベクトルを返す。零ベクトルとの内積は0であり、
    // どの反射の式を通しても寄与が出ない(窓を掛ける前の実装が距離を下から抑えて同じ振る舞いをしていた)。
    result.directionToLight = lightDistance > 0.0 ? toLight / lightDistance : float3(0.0, 0.0, 0.0);
    result.radiance = record.colorAndIntensity.rgb * record.colorAndIntensity.w
        / (squaredDistance + localLightSquaredDistanceEpsilon) * window;
    return true;
}
