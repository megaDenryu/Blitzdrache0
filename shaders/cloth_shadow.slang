// 布だけを描くシャドウマップ生成パスの頂点シェーダー(位置のみ)。フラグメントは空にする
// (深度書き込みだけが目的で、色出力を持たないパスのため。判断35)。
//
// 布の頂点位置は布シミュレーションの結果として世界原点を基準にGPUで作られている。そのため
// このシェーダーは個体変換のストレージバッファも可視ID列も読まず、頂点バッファの位置を
// カメラ相対アンカーで移すだけでライトビュー射影へ渡す。通常の描画対象と同じshadow.slangで
// 布を描くと、布の頂点へ走査順で最初の描画対象の個体変換が掛かり、布の影がどの描画対象が
// 先に来るかで動いてしまう。その依存を断つためにこのシェーダーがある
// (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「描画段階資源の器と布専用シャドウ経路」)。
// 距離区分ごとに1回ずつ発行し、どの距離区分を描いているかはプッシュ定数で受け取る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import shadow_push;

// 布専用のディスクリプタセットレイアウトのbinding3を指す。中身はフレームユニフォームバッファであり、
// 先頭のcascadeLightViewProjectionだけを宣言する(判断36: scene.slangのFrameUniformも先頭に同じ配列を置く)。
struct FrameUniformLight
{
    column_major float4x4 cascadeLightViewProjection[4];
};

[[vk::binding(3, 0)]]
ConstantBuffer<FrameUniformLight> frameUniformLight;

// 頂点入力は位置のみ。バインドする布頂点バッファはscene.slangと同じ48バイトの頂点構造体である。
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
    float3 worldPosition = applyShadowAnchor(input.position);
    output.position = mul(frameUniformLight.cascadeLightViewProjection[currentCascadeIndex()], float4(worldPosition, 1.0));
    return output;
}

[shader("fragment")]
void fragmentMain()
{
}
