// 材質レコードのストレージバッファ(材質のセットのbinding0)。1つの資源表世代が、その世代へ積んだ全材質のレコードを並べた列を
// 1本持ち、描画定数が運ぶ添字でその中の1件を参照する。添字が意味を持つのはその世代の中だけであり、
// 世代を跨いで持ち回してよいのは大域の材質IDのほうである。
// featureBitsはテクスチャの有無を表す。ビットが立っていない役割のスロットも実在する正準フォールバックの画像を指すため、
// 画素段は分岐なしで標本してよい(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「材質レコードとテクスチャ台帳」)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct MaterialRecord
{
    float4 baseColorFactor;
    float4 metallicRoughnessFactor;
    uint featureBits;
    uint baseColorTextureSlot;
    uint metallicRoughnessTextureSlot;
    uint normalTextureSlot;
};

[[vk::binding(0, 2)]]
StructuredBuffer<MaterialRecord> materialRecords;
