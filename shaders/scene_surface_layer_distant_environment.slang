// 遠方環境の契約で、地表の層の重ね合わせの材質を描く画素段。定数近似の契約との違いは間接光の項の出どころだけであり、
// アルベドの組み立ては`surface_layer_albedo.slang`、残りの表面値と直接光の合成は`scene.slang`と同じモジュールを共有する。
// 頂点段は`scene.slang`のものをそのまま使うため、このファイルは画素段のエントリだけを持つ。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import direct_light_shading;
import indirect_distant_environment;
import lighting_query;
import local_visibility_apply;
import material_record;
import motion_vector;
import scene_diagnostic;
import scene_io;
import scene_surface;
import surface_layer_albedo;

// 色合いの係数は`scene.slang`のホットリロード検証だけが書き換える対象であり、この変種では常に等倍である。
static const float3 surfaceLayerDistantEnvironmentTint = float3(1.0, 1.0, 1.0);

[shader("fragment")]
ColorAndMotionOutput fragmentMain(VertexOutput input)
{
    MaterialRecord materialRecord = sceneMaterialRecord();
    float3 albedo = surfaceLayerAlbedoOf(materialRecord, input.uv, surfaceLayerDistantEnvironmentTint);
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
    float3 ambientTerm = distantEnvironmentIndirect(
        surface.normal, surface.viewDir, lightingQueryHeader.sunDirection.xyz,
        surface.albedo, surface.metallic, surface.roughness, localVisibilityOfScreenPosition(input.position));
    float3 litColor = shadeLitSurface(
        ambientTerm, surface.albedo, surface.metallic, surface.roughness, surface.normal, surface.viewDir,
        input.relativePosition, surface.viewDepth, uint2(input.position.xy));
    return sceneColorAndMotion(litColor, input);
}
