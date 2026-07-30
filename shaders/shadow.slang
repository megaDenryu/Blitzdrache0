// シャドウマップ生成パスの頂点シェーダー(位置のみ)。フラグメントは空にする
// (深度書き込みだけが目的で、色出力を持たないパスのため。判断35)。
// 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断35」。
//
// 個体別LODを描くため、シーンパスと同じ可視ID列(binding7)を同じ段の範囲の開始から参照する。
// 注意: このパスが描くインスタンス数は常にその段の全個体である。可視集合をシャドウへ流用しないという不変条件は、
// シェーダーがこの列を読まないことでなく、書き込みの地点で列が全個体の並べ替えであることを検査し、
// 発行のインスタンス数へ段の可視数でなく段の個体数を渡すことで守る
// (参照: `_doc/設計/植生インスタンスと物量計測.md`「可視判定」)。
// 帯ごとに1回ずつ発行し、どの帯を描いているかはプッシュ定数で受け取る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import instance_transform;
import shadow_push;
import visible_id;

// scene.slangのFrameUniformと同じbinding3を指すが、先頭のcascadeLightViewProjectionだけを
// 宣言する(判断36: 両ファイルとも先頭配置のため、この構造体の1フィールド分の
// オフセットがscene.slang側と一致する)。
struct FrameUniformLight
{
    column_major float4x4 cascadeLightViewProjection[4];
};

[[vk::binding(3, 0)]]
ConstantBuffer<FrameUniformLight> frameUniformLight;


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
    InstanceTransform transform = instanceTransforms[resolveInstanceIndex(instanceIndex, startInstance)];
    float3 worldPosition = applyShadowAnchor(mul(transform.localToWorld, float4(input.position, 1.0)).xyz);
    output.position = mul(frameUniformLight.cascadeLightViewProjection[currentCascadeIndex()], float4(worldPosition, 1.0));
    return output;
}

[shader("fragment")]
void fragmentMain()
{
}
