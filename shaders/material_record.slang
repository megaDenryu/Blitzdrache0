// 材質レコードのストレージバッファ(材質のセットのbinding0)。1つの描画対象が自分の材質スロットのぶんだけレコードを並べた列を1本持ち、
// 描画定数が運ぶ添字でその中の1件を参照する。材質スロットごとにバッファを複製しないため、同じ対象のどのスロットの
// ディスクリプタセットも同じバッファを結ぶ。
// 添字が意味を持つのはその描画対象の列の中だけである。大域の材質IDと資源表世代は段4で入れる
// (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct MaterialRecord
{
    float4 baseColorFactor;
    float4 metallicRoughnessFactor;
};

[[vk::binding(0, 2)]]
StructuredBuffer<MaterialRecord> materialRecords;
