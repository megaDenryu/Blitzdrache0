// シャドウマップ生成パスの頂点シェーダー(位置のみ)。フラグメントは空にする
// (深度書き込みだけが目的で、色出力を持たないパスのため。判断35)。
// 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断35」。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// scene.slangのFrameUniformと同じbinding3を指すが、先頭のlightViewProjectionだけを
// 宣言する(判断36: 両ファイルとも先頭配置のため、この構造体の1フィールド分の
// オフセットがscene.slang側と一致する)。
struct FrameUniformLight
{
    column_major float4x4 lightViewProjection;
};

[[vk::binding(3, 0)]]
ConstantBuffer<FrameUniformLight> frameUniformLight;

struct VertexInput
{
    [[vk::location(0)]] float3 position : POSITION;
};

struct VertexOutput
{
    float4 position : SV_Position;
};

[shader("vertex")]
VertexOutput vertexMain(VertexInput input)
{
    VertexOutput output;
    output.position = mul(frameUniformLight.lightViewProjection, float4(input.position, 1.0));
    return output;
}

[shader("fragment")]
void fragmentMain()
{
}
