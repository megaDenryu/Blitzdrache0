// 布の描画パスの頂点入出力構造体。頂点段の出力と、契約ごとに分かれた画素段の入力が同じ宣言を共有するため、
// 独立したファイルに置く。
// 頂点入力は3属性(位置・法線・uv)。接線は消費しないためパイプライン側も宣言しない(頂点属性選択::布用)。
// relativePositionはカメラ大域原点を引いたカメラ相対フレームの位置である(絶対ワールド座標ではない)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct VertexInput
{
    [[vk::location(0)]] float3 position : POSITION;
    [[vk::location(1)]] float3 normal : NORMAL;
    [[vk::location(2)]] float2 uv : TEXCOORD0;
};

struct VertexOutput
{
    float4 position : SV_Position;
    float3 relativePosition : TEXCOORD1;
    float3 normal : TEXCOORD2;
    float2 uv : TEXCOORD0;
};
