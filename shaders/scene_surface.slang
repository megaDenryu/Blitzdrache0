// シーン描画の画素段が材質と頂点の補間から表面の値を組み立てる工程。返すのは間接光の供給元に依らない
// 「その画素の材質がどう見えるか」だけであり、光を1つも足さない。
// 照明問い合わせ契約ごとに画素段のエントリが分かれるため、両方のエントリがこの1つの組み立てを共有する
// (参照: `_doc/設計/放射輝度問い合わせ階層.md`「3-Icの消費式と実装段割り」)。
// 材質のセット(set2)の宣言をここへ置くのは、表面の値を作るのがこの表を読む唯一の工程だからである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import cascade_shadow;
import material_record;
import pbr;
import scene_draw_constants;
import scene_io;
import view_uniform;

// 材質のセットのbinding1: 資源表世代が常駐させた全テクスチャの表。binding2は表全体を読む1つの固定サンプラーである。
// 常駐枚数に満たない残りの要素は書き込まれない(部分束縛)。
// 要素数はCPU側の`テクスチャ表レイアウト容量`と同じ値でなければならず、`cargo xtask conform`が突き合わせる。
// 要素数を書かない実行時配列にできないのは、SPIR-Vの妥当性検査がUniformConstantの実行時配列を拒むためである。
// 隣り合うプリミティブが別のスロットを参照するため、添字はNonUniformResourceIndexで非一様であることを明示する。
static const uint materialTextureTableCapacity = 256;
[[vk::binding(1, 2)]] Texture2D materialTextures[materialTextureTableCapacity];
[[vk::binding(2, 2)]] SamplerState materialSampler;

// テクスチャを持たない役割のスロットも実在する正準フォールバックの画像を指すため、特徴ビットで分岐せず常に標本する。
float4 sampleMaterialTexture(uint slot, float2 uv)
{
    return materialTextures[NonUniformResourceIndex(slot)].Sample(materialSampler, uv);
}

// 光を足す前の表面。法線は法線マップ適用後のワールドの向き、視線は表面からカメラへ向かう単位ベクトルである。
struct SceneSurface
{
    float3 albedo;
    float3 normal;
    float3 viewDir;
    float metallic;
    float roughness;
    float viewDepth;
};

// この描画発行が読む材質の1件。どの1件かはプッシュ定数の添字が決める。
MaterialRecord sceneMaterialRecord()
{
    return materialRecords[sceneDrawConstants.materialRecordIndex];
}

// 色合いの係数を引数で受けるのは、その値がエントリごとの定数であり、ホットリロードの検証が
// `shaders/scene.slang`の宣言を文字列で書き換える対象だからである。
float3 sceneAlbedoOf(MaterialRecord materialRecord, float2 uv, float3 tint)
{
    return sampleMaterialTexture(materialRecord.baseColorTextureSlot, uv).rgb * materialRecord.baseColorFactor.rgb * tint;
}

// ベースカラーを引数で受けるのは、照明が無効なフレームがベースカラーだけを求めて早く返るためである。
// 残りの2枚のテクスチャをそのときに標本しないことがこの分け方の意味である。
SceneSurface sceneSurfaceOf(MaterialRecord materialRecord, VertexOutput input, float3 albedo)
{
    // glTF規約: metallicRoughnessテクスチャはG=粗さ・B=金属度。
    float2 metallicRoughnessSample = sampleMaterialTexture(materialRecord.metallicRoughnessTextureSlot, input.uv).gb;
    float3 sampledNormal = sampleMaterialTexture(materialRecord.normalTextureSlot, input.uv).rgb * 2.0 - 1.0;
    SceneSurface surface;
    surface.albedo = albedo;
    surface.roughness = saturate(metallicRoughnessSample.x * materialRecord.metallicRoughnessFactor.y);
    surface.metallic = saturate(metallicRoughnessSample.y * materialRecord.metallicRoughnessFactor.x);
    surface.normal = applyNormalMap(input.normal, input.tangent, sampledNormal);
    surface.viewDir = normalize(viewUniform.cameraRelativePosition.xyz - input.relativePosition);
    surface.viewDepth = viewDepthOf(input.relativePosition, viewUniform.cameraRelativePosition.xyz, viewUniform.cameraForward.xyz);
    return surface;
}
