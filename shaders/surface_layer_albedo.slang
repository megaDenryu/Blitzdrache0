// 地表の層の重ね合わせのアルベドの組み立て。担うのは「重みの画像と4枚のタイルから、その画素の色を作る」ことだけであり、
// 光を1つも足さない。定数近似の契約と遠方環境の契約の画素段が同じこの1つを取り込む。
// 参照: `_doc/設計/マルチマテリアルと材質境界.md`「地表の層の重ね合わせ」
//
// 重みの標本を半テクセル寄せするのは、地形のUVが格子点の端で0と1にちょうど届き、繰り返しの標本の設定では
// 端の1行が反対側の端と混ざるためである。寄せ方は uv' = (uv * (N - 1) + 0.5) / N であり、Nは重みの画像の寸法である。
// これにより格子点の添字jがちょうどテクセルjの中心を指す。Nを材質レコードで運ばないのは、画像そのものが寸法を答えるためである。
// 南北の向きの合わせは焼く側が済ませている。重みの格子はZが0の行がチャンクの北端であり、テクスチャの行はその逆順である。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import material_record;
import scene_surface;

// CPU側の正本は`crates/blitz_render/src/material/layer_count.rs`であり、`cargo xtask conform`が突き合わせる。
static const uint surfaceLayerCount = 4;

// 重みの合計が零に落ちた画素で色が発散しないための下限。塗られた重みの合計は1付近であり、この値には届かない。
static const float surfaceLayerWeightSumFloor = 1e-4;

float2 surfaceLayerWeightTexcoord(uint slot, float2 uv)
{
    uint width;
    uint height;
    materialTextures[NonUniformResourceIndex(slot)].GetDimensions(width, height);
    float2 texelCount = float2(float(width), float(height));
    return (uv * (texelCount - 1.0) + 0.5) / texelCount;
}

// 層のタイルを重みで混ぜた色。重みを合計で割るのは、混ぜ合わせを凸結合にして明るさが重みの総量で動かないようにするためである。
float3 surfaceLayerAlbedoOf(MaterialRecord materialRecord, float2 uv, float3 tint)
{
    uint weightSlot = materialRecord.surfaceLayerWeightTextureSlot;
    float4 weights = materialTextures[NonUniformResourceIndex(weightSlot)]
                         .Sample(materialSampler, surfaceLayerWeightTexcoord(weightSlot, uv));
    float weightSum = weights.r + weights.g + weights.b + weights.a;
    float3 blended = float3(0.0, 0.0, 0.0);
    for (uint layer = 0; layer < surfaceLayerCount; ++layer)
    {
        float2 tileUv = uv * materialRecord.surfaceLayerTileScale[layer];
        blended += sampleMaterialTexture(materialRecord.surfaceLayerTileTextureSlots[layer], tileUv).rgb * weights[layer];
    }
    return blended / max(weightSum, surfaceLayerWeightSumFloor) * tint;
}
