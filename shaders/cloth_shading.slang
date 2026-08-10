// 布の描画の画素段が間接光の供給元に依らず共有する部分。両面ランバートの法線・織り目・直接光の合計を担う。
// 照明問い合わせ契約ごとに画素段のエントリが分かれるため、両方のエントリがこの1つの組み立てを共有する。
//
// 局所光を読むのは、点光源だけで照らされる屋内で布だけが真っ黒に落ちるためである。列挙と減衰は
// `local_light_shading.slang`をシーン描画と共有し、布が持つのは両面ランバートの反射の式だけである。
// シーンと同じ`shadeLitSurface`を呼ばないのは、あちらが金属度と粗さを持つPBRの式であり、布の見た目そのものを
// 別のものへ変えてしまうためである(参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断j: 布も局所光を消費する」)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import cascade_shadow;
import cascade_shadow_uniform;
import direct_light_shading;
import lighting_query;
import local_light_shading;
import view_uniform;

static const float3 clothColor = float3(0.55, 0.1, 0.12);

// 両面ランバート: 裏面から見た場合は法線を反転して照らす(布は薄い面のため)。
float3 clothSurfaceNormal(float3 inputNormal, bool isFrontFace)
{
    return normalize(inputNormal) * (isFrontFace ? 1.0 : -1.0);
}

// uvからごく薄い格子模様を付けて織り目風にする(uv消費の実用も兼ねる)。
float clothWeave(float2 uv)
{
    return 0.94 + 0.06 * abs(sin(uv.x * 100.0) * sin(uv.y * 100.0));
}

// 方向光の列を件数ぶん足す。
float3 clothDirectionalLight(float3 normal, float3 relativePosition, float viewDepth)
{
    float3 direct = float3(0.0, 0.0, 0.0);
    for (uint directionalIndex = 0; directionalIndex < lightingQueryHeader.directionalLightCount; directionalIndex++)
    {
        DirectionalLightRecord record = directionalLightRecords[directionalIndex];
        // レコードの方向は正規化済みだが、正規化を残すのは段5が絵を1画素も変えない移行だったからである。
        float3 toLight = normalize(record.directionToLightAndIntensity.xyz);
        float lambert = max(dot(normal, toLight), 0.0);
        float visibility = directionalVisibilityFactor(record, relativePosition, viewDepth);
        direct += record.color.rgb * record.directionToLightAndIntensity.w * lambert * visibility;
    }
    return direct;
}

// その画素のセルへ割り当てられた局所光を件数ぶん足す。影は持たないため可視度を掛けない。
float3 clothLocalLight(float3 normal, float3 relativePosition, float viewDepth, uint2 screenPixel)
{
    ClusterCell cell = localLightClusterCellOfPixel(screenPixel, viewDepth);
    float3 direct = float3(0.0, 0.0, 0.0);
    for (uint slot = 0; slot < cell.count; slot++)
    {
        LocalLightSample light;
        if (!localLightSampleOf(localLightRecordOfClusterSlot(cell, slot), relativePosition, light))
        {
            continue;
        }
        direct += light.radiance * max(dot(normal, light.directionToLight), 0.0);
    }
    return direct;
}

// 方向光と局所光の合計。方向光を先に足すのは、この順序がシーン描画の合成と同じだからである。
float3 clothDirectLight(float3 normal, float3 relativePosition, uint2 screenPixel)
{
    float viewDepth = viewDepthOf(relativePosition, viewUniform.cameraRelativePosition.xyz, viewUniform.cameraForward.xyz);
    return clothDirectionalLight(normal, relativePosition, viewDepth)
        + clothLocalLight(normal, relativePosition, viewDepth, screenPixel);
}
