// 描画ごとに頂点ステージへ送るカメラ相対アンカー。頂点のワールド位置へ加算すると、
// カメラ大域原点を引いた相対フレームの位置になる(参照: `_doc/設計/地形とカメラ相対描画.md`)。
// シーン・シャドウ・布・粒子の各頂点シェーダーが同じ16バイトを読むため、宣言をこのファイル1つに集める。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct RelativeAnchor
{
    float3 position;
    float padding;
};

[[vk::push_constant]]
RelativeAnchor relativeAnchor;

// アンカーを加算してカメラ相対のワールド位置にする。
float3 applyRelativeAnchor(float3 worldPosition)
{
    return worldPosition + relativeAnchor.position;
}
