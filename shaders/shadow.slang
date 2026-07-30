// シャドウマップ生成パスの頂点シェーダー(位置のみ)。フラグメントは空にする
// (深度書き込みだけが目的で、色出力を持たないパスのため。判断35)。
// 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断35」。
//
// 個体別LODを描くため、シーンパスと同じ可視ID列(binding7)を参照する。読み始める位置は帯ごとに違い、
// その帯の区間の中のこの段の開始である。
// 注意: 描くインスタンス数はその帯のライト視錐台が通した個体の数であり、シーン側や他の帯の可視数ではない。
// カメラの可視集合を影へ流用しないという不変条件は、シェーダーがこの列を読まないことでなく、書き込みの地点で
// 段別個体数の合計が個体数と一致することとパスごとに重複が無いことを検査し、発行のインスタンス数へその帯の区間の
// 描画数を渡すことで守る
// (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「可視ID列の帯別区間(4区分並べの後継)」)。
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
