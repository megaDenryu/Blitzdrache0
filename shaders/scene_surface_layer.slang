// 定数近似の契約で、地表の層の重ね合わせの材質を描く画素段。標準金属粗さPBRの画素段(`scene.slang`)との違いは
// アルベドの組み立てだけであり、表面値の残り・診断の枝・直接光の合成はすべて同じモジュールを共有する。
// 頂点段は`scene.slang`のものをそのまま使うため、このファイルは画素段のエントリだけを持つ。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import direct_light_shading;
import lighting_query;
import local_visibility_apply;
import material_record;
import motion_vector;
import scene_diagnostic;
import scene_io;
import scene_surface;
import surface_layer_albedo;

// 色合いの係数は`scene.slang`のホットリロード検証だけが書き換える対象であり、この変種では常に等倍である。
static const float3 surfaceLayerTint = float3(1.0, 1.0, 1.0);

[shader("fragment")]
ColorAndMotionOutput fragmentMain(VertexOutput input)
{
    MaterialRecord materialRecord = sceneMaterialRecord();
    float3 albedo = surfaceLayerAlbedoOf(materialRecord, input.uv, surfaceLayerTint);
    if (lightingQueryHeader.lightingEnabled == 0)
    {
        return sceneColorAndMotion(albedo, input);
    }
    SceneSurface surface = sceneSurfaceOf(materialRecord, input, albedo);

    float3 diagnosticColor;
    if (sceneDiagnosticColor(input, surface.viewDepth, diagnosticColor))
    {
        return sceneColorAndMotion(diagnosticColor, input);
    }
    float3 ambientTerm = applyLocalVisibility(
        surface.albedo * lightingQueryHeader.constantIndirectFactor, localVisibilityOfScreenPosition(input.position));
    float3 litColor = shadeLitSurface(
        ambientTerm, surface.albedo, surface.metallic, surface.roughness, surface.normal, surface.viewDir,
        input.relativePosition, surface.viewDepth, uint2(input.position.xy));
    return sceneColorAndMotion(litColor, input);
}
