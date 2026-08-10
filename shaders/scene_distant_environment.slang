// 遠方環境の契約のシーン描画の画素段。定数近似の契約との違いは間接光の項の出どころだけであり、
// 表面値の組み立て・診断の枝・直接光の合成はすべて`scene.slang`と同じモジュールを共有する。
// 頂点段は`scene.slang`のものをそのまま使うため、このファイルは画素段のエントリだけを持つ。
// 間接光は拡散照度・鏡面畳込み・反射率積分表の3つから求める(`indirect_distant_environment.slang`)。
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

// 色合いの係数は`scene.slang`のホットリロード検証だけが書き換える対象であり、この契約では常に等倍である。
static const float3 distantEnvironmentTint = float3(1.0, 1.0, 1.0);

[shader("fragment")]
ColorAndMotionOutput fragmentMain(VertexOutput input)
{
    MaterialRecord materialRecord = sceneMaterialRecord();
    float3 albedo = sceneAlbedoOf(materialRecord, input.uv, distantEnvironmentTint);
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
