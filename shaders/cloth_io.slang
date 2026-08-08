// 布の描画パスの頂点入出力構造体。頂点段の出力と、契約ごとに分かれた画素段の入力が同じ宣言を共有するため、
// 独立したファイルに置く。
// 頂点入力は3属性(位置・法線・uv)。接線は消費しないためパイプライン側も宣言しない(頂点属性選択::布用)。
// relativePositionはカメラ大域原点を引いたカメラ相対フレームの位置である(絶対ワールド座標ではない)。
// 2つの契約の画素段が同じ2枚の添付へ書くため、色と動きベクトルをまとめる1本もここに置く。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import motion_vector;

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
    // 動きベクトルの材料。画素段のSV_Positionは窓座標へ変換済みで同次座標を持たないため、今のクリップ位置も別に運ぶ。
    float4 currentClipPosition : TEXCOORD4;
    // 前フレームのずらし無しビュー射影が写したクリップ位置。布の変形は含まずカメラの運動だけを表す
    // (布とスキンの前フレーム頂点位置は第2弾の課題である。参照: `_doc/設計/時間再構成.md`「将来の拡張」)。
    float4 previousClipPosition : TEXCOORD5;
};

// 布の色とこの画素の動きベクトルをまとめた出力。定数近似と遠方環境の両方の画素段が同じこの1本を通す。
ColorAndMotionOutput clothColorAndMotion(float3 color, VertexOutput input)
{
    return colorAndMotion(color, input.currentClipPosition, input.previousClipPosition);
}
