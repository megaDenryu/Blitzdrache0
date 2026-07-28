// 個体変換のストレージバッファ(binding6)。シーンとシャドウの両方の頂点シェーダーが同じ要素を同じ刻みで引くため、
// 宣言をこのファイル1つに集める。描画対象ユニフォーム(binding5)の先頭も同じ配置であり、個体が1体だけの対象は
// binding6がそのユニフォームの先頭を指す。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct InstanceTransform
{
    column_major float4x4 localToWorld;
    column_major float3x3 normalLocalToWorld;
};

[[vk::binding(6, 0)]]
StructuredBuffer<InstanceTransform> instanceTransforms;
