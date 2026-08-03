// シーン描画の画素段が本番の色の代わりに検収へ出す診断の枝。受け取るのは表面の位置と視線方向の深さ、
// 返すのは診断の色を出すかどうかと、その色である。
// 照明問い合わせ契約ごとに画素段のエントリが分かれるため、両方のエントリがこの1つの枝を共有する。
// 診断は間接光の供給元を1つも見ない。絵の最終判定は本番の色で行い、ここは検収が画素から読む補助である。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import cascade_shadow_uniform;
import direct_light_shading;
import lighting_query;
import pixel_diagnostic;
import scene_io;
import view_uniform;

// 診断の枝が読む影係数。診断は方向光1件ぶんの遮蔽を出す補助であり、光が0件なら影なしとして描く。
float debugVisibilityFactor(float3 relativePosition, float viewDepth)
{
    if (lightingQueryHeader.directionalLightCount == 0)
    {
        return 1.0;
    }
    return directionalVisibilityFactor(directionalLightRecords[0], relativePosition, viewDepth);
}

// cascadeDebugAndTexelSize.xは診断の種別。0でないとき、本番の色の代わりに検収が画素から読む値を出す。
bool sceneDiagnosticColor(VertexOutput input, float viewDepth, out float3 diagnosticColor)
{
    return pixelDiagnosticColor(
        cascadeShadowUniform.cascadeDebugAndTexelSize.x, viewDepth, cascadeShadowUniform.cascadeSplitDepths.xyz,
        cascadeShadowUniform.cascadeTransitionWidths.xyz, debugVisibilityFactor(input.relativePosition, viewDepth),
        length(input.relativePosition - viewUniform.cameraRelativePosition.xyz), diagnosticColor);
}
