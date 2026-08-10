// クラスタ格子のセルへ局所光を割り当てるコンピュート。1つのスレッドが1つのセルを受け持ち、そのセルの錐台と
// 交わりうる局所光の添字を、そのセルへ割り当てられた区間へ宣言の順に書く。
// 束縛するのは生成側のセットであり、照明問い合わせのセット(set3)を1つも束縛しない。set3の段の指定を画素段へ
// 限る方針を保つためである(参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断e」)。
// セルごとに同じ長さの区間を割り当てるため、同じ入力からは常に同じ並びが出る。書ける件数の上限は世界が
// 宣言できる光の件数と同じであり、打ち切りは起こらない。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import cluster_cell_frustum;
import cluster_mapping;
import cluster_sphere_intersection;
import lighting_query_records;
import local_light_records;

// セル1つが指せる局所光の件数の上限。CPU正本の`局所光源の上限件数`と同じ値でなければならない。
static const uint clusterLightsPerCellCapacity = 64;
// 1つの班が受け持つセルの数。班の数は`セルの総数 / この値`の切り上げである。
static const uint clusterAssignmentThreadsPerGroup = 64;

// カメラ相対ワールドの位置をビュー空間へ写す変換の3行。カメラ相対フレームでは視点が原点にあるため
// 平行移動の成分は0だが、行をそのまま運ぶことで写し取りの取り違えを作らない。
struct ClusterAssignmentViewRows
{
    float4 firstRow;
    float4 secondRow;
    float4 thirdRow;
};

[[vk::push_constant]] ClusterAssignmentViewRows clusterAssignmentViewRows;

[[vk::binding(0, 0)]] ConstantBuffer<LightingQueryHeader> clusterAssignmentHeader;
[[vk::binding(1, 0)]] StructuredBuffer<LocalLightRecord> clusterAssignmentLocalLights;
[[vk::binding(2, 0)]] RWStructuredBuffer<ClusterCell> clusterAssignmentCells;
[[vk::binding(3, 0)]] RWStructuredBuffer<uint> clusterAssignmentLightIndices;

float3 clusterAssignmentViewPositionOf(float3 cameraRelativePosition)
{
    float4 homogeneous = float4(cameraRelativePosition, 1.0);
    return float3(
        dot(clusterAssignmentViewRows.firstRow, homogeneous),
        dot(clusterAssignmentViewRows.secondRow, homogeneous),
        dot(clusterAssignmentViewRows.thirdRow, homogeneous));
}

[shader("compute")]
[numthreads(clusterAssignmentThreadsPerGroup, 1, 1)]
void computeMain(uint3 threadIdentifier : SV_DispatchThreadID)
{
    uint3 divisions = clusterAssignmentHeader.clusterGridDivisions.xyz;
    uint flatIndex = threadIdentifier.x;
    if (flatIndex >= divisions.x * divisions.y * divisions.z)
    {
        return;
    }
    ClusterCellFrustum frustum = clusterCellFrustumOf(
        clusterCellOfFlatIndex(flatIndex, divisions),
        divisions,
        clusterAssignmentHeader.clusterScreenPixelExtent.xy,
        clusterAssignmentHeader.clusterDepthRangeAndHalfAngleTangent.xy,
        clusterAssignmentHeader.clusterDepthRangeAndHalfAngleTangent.zw);

    uint start = clusterLightIndexStart(flatIndex, clusterLightsPerCellCapacity);
    uint written = 0;
    for (uint lightIndex = 0; lightIndex < clusterAssignmentHeader.localLightCount; lightIndex++)
    {
        LocalLightRecord record = clusterAssignmentLocalLights[lightIndex];
        // 光種の枝を明示する。種別を読まずに球の判定を当てると、光種が増えたときに別の種別の光が
        // 点光と同じ範囲で黙って割り当てられる。
        if (record.kind.x != localLightKindPoint)
        {
            continue;
        }
        float3 viewCenter = clusterAssignmentViewPositionOf(record.cameraRelativePosition.xyz);
        if (!clusterFrustumIntersectsSphere(frustum, viewCenter, record.influenceRadius))
        {
            continue;
        }
        clusterAssignmentLightIndices[start + written] = lightIndex;
        written++;
    }

    ClusterCell cell;
    cell.start = start;
    cell.count = written;
    clusterAssignmentCells[flatIndex] = cell;
}
