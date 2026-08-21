// 材質レコードのストレージバッファ(材質のセットのbinding0)。1つの資源表世代が、その世代へ積んだ全材質のレコードを並べた列を
// 1本持ち、描画定数が運ぶ添字でその中の1件を参照する。添字が意味を持つのはその世代の中だけであり、
// 世代を跨いで持ち回してよいのは大域の材質IDのほうである。
// featureBitsはテクスチャの有無を表す。ビットが立っていない役割のスロットも実在する正準フォールバックの画像を指すため、
// 画素段は分岐なしで標本してよい(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「材質レコードとテクスチャ台帳」)。
// スロットの欄がシェーディングモデルに依らず8本あるのは、レコードの並びを1つに保つためである。標準金属粗さPBRは先頭の3本を、
// 地表の層の重ね合わせは残りの5本を読み、読まない欄は正準フォールバックを指す。
// 注意: 末尾のpaddingを削ってはならない。ストレージバッファの要素の刻みは最大整列(float4の16バイト)の倍数でなければならず、
// 値の合計84バイトのままでは2件目以降の読み出し位置がずれる。CPU側の正本は
// `crates/blitz_render/src/vulkan/material_record/bytes.rs`であり、並びの一致を単体テストが見る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct MaterialRecord
{
    float4 baseColorFactor;
    float4 metallicRoughnessFactor;
    float4 surfaceLayerTileScale;
    uint featureBits;
    uint baseColorTextureSlot;
    uint metallicRoughnessTextureSlot;
    uint normalTextureSlot;
    uint surfaceLayerWeightTextureSlot;
    uint surfaceLayerTileTextureSlots[4];
    uint padding[3];
};

[[vk::binding(0, 2)]]
StructuredBuffer<MaterialRecord> materialRecords;
