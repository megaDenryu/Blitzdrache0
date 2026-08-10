// 点光源の影の記録の1発行ぶんに頂点ステージへ送るプッシュ定数。カメラ相対の基準原点16バイトに、その面の
// ライトビュー射影64バイトを足した80バイトである。
//
// 行列をプッシュ定数で運ぶのは、面ごとの行列が最大24本あり、そのうち1本だけをその発行が読むためである。
// 24本を定数バッファへ載せて添字で選ぶ形にすると、灯の件数が変わるたびにバッファの形が動き、
// 影を1件も持たない世界も24本ぶんの領域を持つことになる。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct PointLightShadowPush
{
    float3 position;
    float padding;
    // column_major明示(CPU側は列優先配列)。その面の投影の契約が組み立てたライトビュー射影である。
    column_major float4x4 faceLightViewProjection;
};

[[vk::push_constant]]
PointLightShadowPush pointLightShadowPush;

// 基準原点を加算してカメラ相対のワールド位置にする。
float3 applyPointLightShadowAnchor(float3 worldPosition)
{
    return worldPosition + pointLightShadowPush.position;
}

// この発行が描いている面のライトビュー射影。
float4x4 currentFaceLightViewProjection()
{
    return pointLightShadowPush.faceLightViewProjection;
}
