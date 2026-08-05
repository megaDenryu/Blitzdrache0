// 定数近似の契約のシーン描画。PBR metallic-roughnessモデル(方向光+局所光+定数間接近似+法線マップ)と、
// 方向光の多段シャドウマップ(照明問い合わせのセットの比較サンプラー、PCF 3x3)の合成(判断23〜26・35)。
// 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`。シェーダー内識別子はASCII。
// 頂点段はこの1本を両方の契約が共有する。遠方環境の契約の画素段は`scene_distant_environment.slang`にあり、
// 表面値の組み立ては`scene_surface.slang`、診断の枝は`scene_diagnostic.slang`を両方が取り込む。
// PBR関数群は`pbr.slang`、直接光の列挙と合成は`direct_light_shading.slang`、距離区分の選択と境界ブレンドは`cascade_shadow.slang`にある。
// ビューの定数は`view_uniform.slang`、影の定数は`cascade_shadow_uniform.slang`、直接光と定数間接近似は`lighting_query.slang`が唯一の宣言を持つ。
// 材質の係数とテクスチャスロットは`material_record.slang`のストレージバッファにあり、どの1件を読むかは
// `scene_draw_constants.slang`のプッシュ定数が運ぶ添字が決める。テクスチャは材質のセットの索引表から読む。

import direct_light_shading;
import instance_transform;
import lighting_query;
import local_visibility_apply;
import material_record;
import scene_diagnostic;
import scene_draw_constants;
import scene_io;
import scene_surface;
import view_uniform;
import visible_id;

// 注意: この1行の表記はスモークのホットリロード検証が文字列置換で書き換える対象。表記を変えると検証が壊れるため、常にこの形(static const float3 = float3(x, y, z);)を保つ。
static const float3 TINT = float3(1.0, 1.0, 1.0);

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
    MaterialRecord materialRecord = sceneMaterialRecord();
    float3 albedo = sceneAlbedoOf(materialRecord, input.uv, TINT);
    if (lightingQueryHeader.lightingEnabled == 0)
    {
        return float4(albedo, 1.0);
    }
    SceneSurface surface = sceneSurfaceOf(materialRecord, input, albedo);

    float3 diagnosticColor;
    if (sceneDiagnosticColor(input, surface.viewDepth, diagnosticColor))
    {
        return float4(diagnosticColor, 1.0);
    }
    float3 ambientTerm = applyLocalVisibility(
        surface.albedo * lightingQueryHeader.constantIndirectFactor, localVisibilityOfScreenPosition(input.position));
    float3 litColor = shadeLitSurface(
        ambientTerm, surface.albedo, surface.metallic, surface.roughness, surface.normal, surface.viewDir,
        input.relativePosition, surface.viewDepth);
    return float4(litColor, 1.0);
}
