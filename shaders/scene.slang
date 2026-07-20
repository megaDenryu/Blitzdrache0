// M3: glTFシーンの頂点(位置+法線+UV)とベースカラーテクスチャを描画する。
// 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断19」「判断20」「判断21」。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct PushConstants
{
    // column_major明示: CPU側は列優先の生配列(blitz_mathのgpu境界用列優先配列)で
    // 送るため、シェーダー側の解釈をそれに一致させる。mul(M, v)は列ベクトルvに対する
    // 標準の行列積になる。
    column_major float4x4 viewProjection;
};

[[vk::push_constant]]
ConstantBuffer<PushConstants> pushConstants;

// set=0 binding=0のcombined image sampler(判断21)。
[[vk::binding(0, 0)]]
Sampler2D baseColorTexture;

struct VertexInput
{
    [[vk::location(0)]] float3 position : POSITION;
    [[vk::location(1)]] float3 normal : NORMAL;
    [[vk::location(2)]] float2 uv : TEXCOORD0;
};

struct VertexOutput
{
    float4 position : SV_Position;
    // 法線はM3では未使用だが、出力に流すだけにしておく(将来のPBR化に備える)。
    float3 normal : NORMAL;
    float2 uv : TEXCOORD0;
};

// 注意: この1行の表記はスモークのホットリロード検証が文字列置換で書き換える対象。
// 表記を変えると検証が壊れるため、常にこの形(static const float3 = float3(x, y, z);)を保つ。
static const float3 TINT = float3(1.0, 1.0, 1.0);

[shader("vertex")]
VertexOutput vertexMain(VertexInput input)
{
    VertexOutput output;
    output.position = mul(pushConstants.viewProjection, float4(input.position, 1.0));
    output.normal = input.normal;
    output.uv = input.uv;
    return output;
}

[shader("fragment")]
float4 fragmentMain(VertexOutput input) : SV_Target
{
    float3 sampled = baseColorTexture.Sample(input.uv).rgb;
    return float4(sampled * TINT, 1.0);
}
