// PBR metallic-roughnessモデル(方向光+局所光+定数間接近似+法線マップ)と、方向光の多段シャドウマップ(照明問い合わせのセットの比較サンプラー、PCF 3x3)の合成(判断23〜26・35)。
// 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`。シェーダー内識別子はASCII。
// エントリポイントはここに置き、PBR関数群は`pbr.slang`、直接光の列挙と合成は`direct_light_shading.slang`、距離区分の選択と境界ブレンドは`cascade_shadow.slang`へ分割してimportする。
// ビューの定数は`view_uniform.slang`、影の定数は`cascade_shadow_uniform.slang`、直接光と定数間接近似は`lighting_query.slang`が唯一の宣言を持つ。
// 材質の係数とテクスチャスロットは`material_record.slang`のストレージバッファにあり、どの1件を読むかは
// `scene_draw_constants.slang`のプッシュ定数が運ぶ添字が決める。テクスチャは材質のセットの索引表から読む。

import cascade_shadow;
import cascade_shadow_uniform;
import direct_light_shading;
import instance_transform;
import lighting_query;
import pixel_diagnostic;
import material_record;
import scene_draw_constants;
import scene_io;
import view_uniform;
import visible_id;

// 材質のセットのbinding1: 資源表世代が常駐させた全テクスチャの表。binding2は表全体を読む1つの固定サンプラーである。
// 常駐枚数に満たない残りの要素は書き込まれない(部分束縛)。
// 要素数はCPU側の`テクスチャ表レイアウト容量`と同じ値でなければならず、`cargo xtask conform`が突き合わせる。
// 要素数を書かない実行時配列にできないのは、SPIR-Vの妥当性検査がUniformConstantの実行時配列を拒むためである。
// 隣り合うプリミティブが別のスロットを参照するため、添字はNonUniformResourceIndexで非一様であることを明示する。
static const uint materialTextureTableCapacity = 256;
[[vk::binding(1, 2)]] Texture2D materialTextures[materialTextureTableCapacity];
[[vk::binding(2, 2)]] SamplerState materialSampler;

// 注意: この1行の表記はスモークのホットリロード検証が文字列置換で書き換える対象。表記を変えると検証が壊れるため、常にこの形(static const float3 = float3(x, y, z);)を保つ。
static const float3 TINT = float3(1.0, 1.0, 1.0);

// テクスチャを持たない役割のスロットも実在する正準フォールバックの画像を指すため、特徴ビットで分岐せず常に標本する。
float4 sampleMaterialTexture(uint slot, float2 uv)
{
    return materialTextures[NonUniformResourceIndex(slot)].Sample(materialSampler, uv);
}

// 診断の枝が読む影係数。診断は方向光1件ぶんの遮蔽を出す補助であり、光が0件なら影なしとして描く。
float debugVisibilityFactor(float3 relativePosition, float viewDepth)
{
    if (lightingQueryHeader.directionalLightCount == 0)
    {
        return 1.0;
    }
    return directionalVisibilityFactor(directionalLightRecords[0], relativePosition, viewDepth);
}

[shader("vertex")]
VertexOutput vertexMain(VertexInput input, uint instanceIndex : SV_InstanceID, uint startInstance : SV_StartInstanceLocation)
{
    VertexOutput output;
    InstanceTransform transform = instanceTransforms[resolveInstanceIndex(instanceIndex, startInstance)];
    float3 relativePosition = applySceneDrawAnchor(mul(transform.localToWorld, float4(input.position, 1.0)).xyz);
    float3 worldNormal = normalize(mul(transform.normalLocalToWorld, input.normal));
    float3 transformedTangent = mul((float3x3)transform.localToWorld, input.tangent.xyz);
    float3 worldTangent = normalize(transformedTangent - worldNormal * dot(worldNormal, transformedTangent));
    output.position = mul(viewUniform.viewProjection, float4(relativePosition, 1.0));
    output.relativePosition = relativePosition;
    output.normal = worldNormal;
    output.tangent = float4(worldTangent, input.tangent.w);
    output.uv = input.uv;
    return output;
}

[shader("fragment")]
float4 fragmentMain(VertexOutput input) : SV_Target
{
    MaterialRecord materialRecord = materialRecords[sceneDrawConstants.materialRecordIndex];
    float3 albedo = sampleMaterialTexture(materialRecord.baseColorTextureSlot, input.uv).rgb * materialRecord.baseColorFactor.rgb * TINT;
    if (lightingQueryHeader.lightingEnabled == 0)
    {
        return float4(albedo, 1.0);
    }
    // glTF規約: metallicRoughnessテクスチャはG=粗さ・B=金属度。
    float2 metallicRoughnessSample = sampleMaterialTexture(materialRecord.metallicRoughnessTextureSlot, input.uv).gb;
    float3 sampledNormal = sampleMaterialTexture(materialRecord.normalTextureSlot, input.uv).rgb * 2.0 - 1.0;
    float viewDepth = viewDepthOf(input.relativePosition, viewUniform.cameraRelativePosition.xyz, viewUniform.cameraForward.xyz);

    // cascadeDebugAndTexelSize.xは診断の種別。0でないとき、本番の色の代わりに検収が画素から読む値を出す。
    float3 diagnosticColor;
    if (pixelDiagnosticColor(
            cascadeShadowUniform.cascadeDebugAndTexelSize.x, viewDepth, cascadeShadowUniform.cascadeSplitDepths.xyz,
            cascadeShadowUniform.cascadeTransitionWidths.xyz, debugVisibilityFactor(input.relativePosition, viewDepth),
            length(input.relativePosition - viewUniform.cameraRelativePosition.xyz), diagnosticColor))
    {
        return float4(diagnosticColor, 1.0);
    }
    float3 litColor = shadeLitSurface(
        albedo, metallicRoughnessSample, materialRecord.metallicRoughnessFactor.x, materialRecord.metallicRoughnessFactor.y,
        sampledNormal, input.normal, input.tangent, input.relativePosition, viewUniform.cameraRelativePosition.xyz, viewDepth);
    return float4(litColor, 1.0);
}
