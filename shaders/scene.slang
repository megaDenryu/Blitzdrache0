// PBR metallic-roughnessモデル(方向光+点光源+環境光+法線マップ)と、方向光の多段シャドウマップ(binding4の比較サンプラー、PCF 3x3)の合成(判断23〜26・35)。
// 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`。シェーダー内識別子はASCII。
// エントリポイントはここに置き、PBR関数群は`pbr.slang`、ライティング合成+シャドウ判定は`lighting.slang`、距離区分の選択と境界ブレンドは`cascade_shadow.slang`へ分割してimportする。
// ビューと直接照明の定数は`view_pass_uniform.slang`、影の定数は`cascade_shadow_uniform.slang`が唯一の宣言を持つ。

import pbr;
import lighting;
import cascade_shadow;
import cascade_shadow_uniform;
import instance_transform;
import relative_anchor;
import scene_io;
import view_pass_uniform;
import visible_id;

struct ObjectUniform
{
    InstanceTransform transform;
    float4 baseColorFactor;
    float4 metallicRoughnessFactor;
};

[[vk::binding(5, 0)]] ConstantBuffer<ObjectUniform> objectUniform;

[[vk::binding(0, 0)]] Sampler2D baseColorTexture;
[[vk::binding(1, 0)]] Sampler2D metallicRoughnessTexture;
[[vk::binding(2, 0)]] Sampler2D normalMapTexture;
// binding4: 多段シャドウマップ(判断35・OW5第5段)。距離区分ごとの層を持つ2D配列テクスチャ+比較サンプラーを同一bindingへ束ねる。
[[vk::binding(4, 0)]] [[vk::combinedImageSampler]] Texture2DArray shadowMapTexture;
[[vk::binding(4, 0)]] [[vk::combinedImageSampler]] SamplerComparisonState shadowMapSampler;

// 注意: この1行の表記はスモークのホットリロード検証が文字列置換で書き換える対象。表記を変えると検証が壊れるため、常にこの形(static const float3 = float3(x, y, z);)を保つ。
static const float3 TINT = float3(1.0, 1.0, 1.0);

[shader("vertex")]
VertexOutput vertexMain(VertexInput input, uint instanceIndex : SV_InstanceID, uint startInstance : SV_StartInstanceLocation)
{
    VertexOutput output;
    InstanceTransform transform = instanceTransforms[resolveInstanceIndex(instanceIndex, startInstance)];
    float3 relativePosition = applyRelativeAnchor(mul(transform.localToWorld, float4(input.position, 1.0)).xyz);
    float3 worldNormal = normalize(mul(transform.normalLocalToWorld, input.normal));
    float3 transformedTangent = mul((float3x3)transform.localToWorld, input.tangent.xyz);
    float3 worldTangent = normalize(transformedTangent - worldNormal * dot(worldNormal, transformedTangent));
    output.position = mul(viewPassUniform.viewProjection, float4(relativePosition, 1.0));
    output.relativePosition = relativePosition;
    output.normal = worldNormal;
    output.tangent = float4(worldTangent, input.tangent.w);
    output.uv = input.uv;
    return output;
}

[shader("fragment")]
float4 fragmentMain(VertexOutput input) : SV_Target
{
    float3 albedo = baseColorTexture.Sample(input.uv).rgb * objectUniform.baseColorFactor.rgb * TINT;
    if (viewPassUniform.ambientAndLightingEnabled.y <= 0.5)
    {
        return float4(albedo, 1.0);
    }
    // glTF規約: metallicRoughnessテクスチャはG=粗さ・B=金属度。shadowFlagsAndTexelSize.xは方向光が影を落とすかであり、0のとき比較サンプリング自体を行わない。
    float2 metallicRoughnessSample = metallicRoughnessTexture.Sample(input.uv).gb;
    float3 sampledNormal = normalMapTexture.Sample(input.uv).rgb * 2.0 - 1.0;
    float viewDepth = viewDepthOf(input.relativePosition, viewPassUniform.cameraRelativePosition.xyz, viewPassUniform.cameraForward.xyz);
    float shadowFactor = cascadeShadowUniform.shadowFlagsAndTexelSize.x > 0.5
        ? computeCascadedShadowFactor(input.relativePosition, viewDepth, cascadeShadowUniform.cascadeLightViewProjection,
              cascadeShadowUniform.cascadeSplitDepths.xyz, cascadeShadowUniform.cascadeTransitionWidths.xyz, shadowMapTexture,
              shadowMapSampler, cascadeShadowUniform.shadowFlagsAndTexelSize.z)
        : 1.0;

    // shadowFlagsAndTexelSize.yは距離区分の可視化指定。1のとき、どの距離区分を参照したかと影の中かだけを色で出す(継ぎ目検収の補助)。
    if (cascadeShadowUniform.shadowFlagsAndTexelSize.y > 0.5)
    {
        float3 debugColor = cascadeDebugPixel(
            viewDepth, cascadeShadowUniform.cascadeSplitDepths.xyz, cascadeShadowUniform.cascadeTransitionWidths.xyz, shadowFactor);
        return float4(debugColor, 1.0);
    }
    float3 litColor = shadeLitSurface(
        albedo, metallicRoughnessSample, objectUniform.metallicRoughnessFactor.x, objectUniform.metallicRoughnessFactor.y,
        sampledNormal, input.normal, input.tangent, input.relativePosition, viewPassUniform.cameraRelativePosition.xyz,
        viewPassUniform.ambientAndLightingEnabled.x, viewPassUniform.directionalLightDirection.xyz, viewPassUniform.directionalLightColor.rgb,
        viewPassUniform.directionalLightColor.w, viewPassUniform.pointLightPosition.xyz, viewPassUniform.pointLightColor.rgb,
        viewPassUniform.pointLightColor.w, shadowFactor);
    return float4(litColor, 1.0);
}
