// 点光源の影の記録パスの頂点シェーダー(位置のみ)。画素段は空にする(深度書き込みだけが目的で色出力を持たない)。
//
// 可視ID列を参照しない点が多段影の記録と違う。可視ID列はパスごとの区間を1本の列へ正典順に並べる形で作られており、
// 面ごとの区間をそこへ足すと列の容量が6倍近くになる。したがって面ごとの候補は別の経路で毎フレーム求め、
// この頂点段は発行が指定した個体の範囲をそのまま描く(参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断n」)。
// 描く個体の絞り込みはコマンド記録の側にあり、灯の影響半径の球と面の錐台で描画対象ごとに判定する。
//
// 注意: この変換の行列式は負であり、三角形の巻き方が反転する。このパスはカリングを行ってはならない(判断m)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import instance_transform;
import point_light_shadow_push;

struct VertexInput
{
    [[vk::location(0)]] float3 position : POSITION;
};

struct VertexOutput
{
    float4 position : SV_Position;
};

[shader("vertex")]
VertexOutput vertexMain(VertexInput input, uint instanceIndex : SV_InstanceID, uint startInstance : SV_StartInstanceLocation)
{
    VertexOutput output;
    InstanceTransform transform = instanceTransforms[instanceIndex + startInstance];
    float3 worldPosition = applyPointLightShadowAnchor(mul(transform.localToWorld, float4(input.position, 1.0)).xyz);
    output.position = mul(currentFaceLightViewProjection(), float4(worldPosition, 1.0));
    return output;
}

[shader("fragment")]
void fragmentMain()
{
}
