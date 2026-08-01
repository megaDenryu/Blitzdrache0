// M9: 布の描画(判断54)。両面ランバート+方向光の多段シャドウの最小シェーダー。
// ディスクリプタはシーンと同じセット(binding3=ビュー・シーンパス定数、binding4=シャドウマップ、binding8=多段影定数)を
// 束縛し、PBRテクスチャ(binding0〜2)は読まない。頂点レイアウトはscene.slangと同じ48バイト。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import cascade_shadow;
import cascade_shadow_uniform;
import relative_anchor;
import view_pass_uniform;

[[vk::binding(4, 0)]] [[vk::combinedImageSampler]] Texture2DArray shadowMapTexture;
[[vk::binding(4, 0)]] [[vk::combinedImageSampler]] SamplerComparisonState shadowMapSampler;

static const float3 clothColor = float3(0.55, 0.1, 0.12);

// 頂点入力は3属性(位置・法線・uv)。接線は消費しないためパイプライン側も宣言しない(頂点属性選択::布用)。
struct VertexInput
{
    [[vk::location(0)]] float3 position : POSITION;
    [[vk::location(1)]] float3 normal : NORMAL;
    [[vk::location(2)]] float2 uv : TEXCOORD0;
};

struct VertexOutput
{
    float4 position : SV_Position;
    float3 relativePosition : TEXCOORD1;
    float3 normal : TEXCOORD2;
    float2 uv : TEXCOORD0;
};

[shader("vertex")]
VertexOutput vertexMain(VertexInput input)
{
    VertexOutput output;
    float3 relativePosition = applyRelativeAnchor(input.position);
    output.position = mul(viewPassUniform.viewProjection, float4(relativePosition, 1.0));
    output.relativePosition = relativePosition;
    output.normal = input.normal;
    output.uv = input.uv;
    return output;
}

// 両面ランバート: 裏面から見た場合は法線を反転して照らす(布は薄い面のため)。
// uvからごく薄い格子模様を付けて織り目風にする(uv消費の実用も兼ねる)。
[shader("fragment")]
float4 fragmentMain(VertexOutput input, bool isFrontFace: SV_IsFrontFace) : SV_Target
{
    float3 normal = normalize(input.normal) * (isFrontFace ? 1.0 : -1.0);
    float3 toLight = -normalize(viewPassUniform.directionalLightDirection.xyz);
    float lambert = max(dot(normal, toLight), 0.0);
    // shadowFlagsAndTexelSize.xは方向光が影を落とすか。scene.slangと同じ判定で、0のとき比較サンプリングを行わない。
    float viewDepth = viewDepthOf(input.relativePosition, viewPassUniform.cameraRelativePosition.xyz, viewPassUniform.cameraForward.xyz);
    float shadow = cascadeShadowUniform.shadowFlagsAndTexelSize.x > 0.5
        ? computeCascadedShadowFactor(input.relativePosition, viewDepth, cascadeShadowUniform.cascadeLightViewProjection,
              cascadeShadowUniform.cascadeSplitDepths.xyz, cascadeShadowUniform.cascadeTransitionWidths.xyz, shadowMapTexture,
              shadowMapSampler, cascadeShadowUniform.shadowFlagsAndTexelSize.z)
        : 1.0;
    // 直接光へ方向光強度(directionalLightColor.w)を掛け、環境光はビュー・シーンパス定数の環境光係数から取る。
    // どちらもscene.slangと同じ読み方であり、これが無いと布だけが夜も方向光で照らされ、夜の環境光にも追従しない。
    float3 direct = viewPassUniform.directionalLightColor.rgb * viewPassUniform.directionalLightColor.w * lambert * shadow;
    float3 ambient = viewPassUniform.ambientAndLightingEnabled.xxx;
    float weave = 0.94 + 0.06 * abs(sin(input.uv.x * 100.0) * sin(input.uv.y * 100.0));
    return float4(clothColor * weave * (direct + ambient), 1.0);
}
