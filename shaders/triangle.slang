// M1: 頂点バッファなしのハードコード三角形。SV_VertexIDから3頂点を生成する。
// 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断6」。
// 注意: シェーダー内識別子はASCII（CLAUDE.md「命名」）。

struct VertexOutput
{
    float4 position : SV_Position;
};

// 注意: この1行の表記はスモークのホットリロード検証が文字列置換で書き換える対象。
// 表記を変えると検証が壊れるため、常にこの形（static const float3 = float3(x, y, z);）を保つ。
static const float3 TRIANGLE_COLOR = float3(1.0, 0.45, 0.1);

[shader("vertex")]
VertexOutput vertexMain(uint vertexIndex : SV_VertexID)
{
    float2 positions[3] = {
        float2(0.0, -0.6),
        float2(0.6, 0.6),
        float2(-0.6, 0.6),
    };

    VertexOutput output;
    output.position = float4(positions[vertexIndex], 0.0, 1.0);
    return output;
}

[shader("fragment")]
float4 fragmentMain(VertexOutput input) : SV_Target
{
    return float4(TRIANGLE_COLOR, 1.0);
}
