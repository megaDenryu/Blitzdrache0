// 個体変換のストレージバッファ(binding6)。シーンとシャドウの両方の頂点シェーダーが同じ要素を同じ刻みで参照するため、
// 宣言をこのファイル1つに集める。個体が1体だけの対象も1要素のこのバッファを結ぶため、他の資源の先頭を個体変換として
// 読む形はどこにも無い。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct InstanceTransform
{
    column_major float4x4 localToWorld;
    column_major float3x3 normalLocalToWorld;
};

[[vk::binding(6, 0)]]
StructuredBuffer<InstanceTransform> instanceTransforms;
