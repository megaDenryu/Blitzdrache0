// 直接光の標本を列挙して表面の色を合成する工程。受け取るのは表面の材質の値と位置、返すのは
// 環境・方向光・局所光を足した色である。光の件数と実体は照明問い合わせのセットのレコード列が持ち、
// このモジュールは列の作られ方を知らない(参照: `_doc/設計/放射輝度問い合わせ階層.md`「A. 直接光標本列挙」)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import cascade_shadow;
import cascade_shadow_uniform;
import lighting_query;
import pbr;

// その方向光の遮蔽をどう判定するかは光ごとの可視性の種別が決める。影なしの枝では比較サンプリング自体を行わない。
float directionalVisibilityFactor(DirectionalLightRecord record, float3 relativePosition, float viewDepth)
{
    if (record.visibility.x != directionalVisibilityCascadedShadow)
    {
        return 1.0;
    }
    return computeCascadedShadowFactor(
        relativePosition, viewDepth, cascadeShadowUniform.cascadeLightViewProjection,
        cascadeShadowUniform.cascadeSplitDepths.xyz, cascadeShadowUniform.cascadeTransitionWidths.xyz,
        shadowMapTexture, shadowMapSampler, cascadeShadowUniform.cascadeDebugAndTexelSize.y);
}

// 件数が0の列は1度も参照しない。現行の直接光は方向光1件と局所光1件であり、どちらも件数1の列として並ぶ。
// 間接光・方向光・局所光の順に足すのは、この順序が絵の正本だからである(足す順を変えると丸めが変わる)。
// 間接光の項を引数で受けるのは、その値をどの供給元から得るかが照明問い合わせ契約ごとに違うためである。
// このモジュールは直接光だけを担い、供給元の枝を1つも知らない
// (参照: `_doc/設計/放射輝度問い合わせ階層.md`「世界の間接照明方針と契約の2枝(3-Ic)」)。
float3 shadeLitSurface(
    float3 ambientTerm, float3 albedo, float metallic, float roughness,
    float3 normal, float3 viewDir,
    float3 relativePosition, float viewDepth)
{
    float3 directionalTerm = float3(0.0, 0.0, 0.0);
    for (uint directionalIndex = 0; directionalIndex < lightingQueryHeader.directionalLightCount; directionalIndex++)
    {
        DirectionalLightRecord record = directionalLightRecords[directionalIndex];
        // レコードの方向は正規化済みだが、正規化を残すのは段5が絵を1画素も変えない移行だからである。
        float3 lightDir = normalize(record.directionToLightAndIntensity.xyz);
        float3 radiance = record.color.rgb * record.directionToLightAndIntensity.w;
        float visibility = directionalVisibilityFactor(record, relativePosition, viewDepth);
        directionalTerm += shadeLight(normal, viewDir, lightDir, radiance, albedo, metallic, roughness) * visibility;
    }

    float3 localTerm = float3(0.0, 0.0, 0.0);
    for (uint localIndex = 0; localIndex < lightingQueryHeader.localLightCount; localIndex++)
    {
        LocalLightRecord record = localLightRecords[localIndex];
        // 光種の枝を明示する。CPU側は局所光レコードを点光としてしか作れないが、種別を読まずに点光の式を当てると、
        // 光種が増えたときに新しい種別の光を黙って点光の減衰で照らしてしまう。
        if (record.kind.x != localLightKindPoint)
        {
            continue;
        }
        float3 toLight = record.cameraRelativePosition.xyz - relativePosition;
        float lightDistance = max(length(toLight), 1e-4);
        float3 lightDir = toLight / lightDistance;
        float3 radiance = record.colorAndIntensity.rgb * record.colorAndIntensity.w / (lightDistance * lightDistance);
        localTerm += shadeLight(normal, viewDir, lightDir, radiance, albedo, metallic, roughness);
    }

    return ambientTerm + directionalTerm + localTerm;
}
