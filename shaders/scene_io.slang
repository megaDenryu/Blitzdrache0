// シーン描画パスの頂点入出力構造体。頂点ステージの出力とフラグメントステージの入力が同じ宣言を共有するため、scene.slangからimportする独立ファイルに置く。
// relativePositionはカメラ大域原点を引いたカメラ相対フレームの位置である(絶対ワールド座標ではない)。

struct VertexInput
{
    [[vk::location(0)]] float3 position : POSITION;
    [[vk::location(1)]] float3 normal : NORMAL;
    [[vk::location(2)]] float4 tangent : TANGENT;
    [[vk::location(3)]] float2 uv : TEXCOORD0;
};

struct VertexOutput
{
    float4 position : SV_Position;
    float3 relativePosition : TEXCOORD1;
    float3 normal : TEXCOORD2;
    float4 tangent : TEXCOORD3;
    float2 uv : TEXCOORD0;
};
