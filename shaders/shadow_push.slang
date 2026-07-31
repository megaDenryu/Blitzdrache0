// シャドウ記録の1発行ぶんに頂点ステージへ送るプッシュ定数。カメラ相対アンカーに距離区分番号を足した20バイトであり、
// 通常のシャドウと布専用シャドウの2本の頂点シェーダーが同じ宣言を読む。
// シーン・粒子が読む relative_anchor.slang の16バイトと別に持つのは、距離区分番号を足した形をシャドウ以外の
// パイプラインレイアウトへ波及させないためである
// (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「カスケードシャドウ(CSM)」)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct ShadowPush
{
    float3 position;
    float padding;
    uint cascadeIndex;
};

[[vk::push_constant]]
ShadowPush shadowPush;

// アンカーを加算してカメラ相対のワールド位置にする。
float3 applyShadowAnchor(float3 worldPosition)
{
    return worldPosition + shadowPush.position;
}

// この発行が描いている距離区分の番号。フレームシェーダー定数の距離区分ごとの行列を参照する添字になる。
uint currentCascadeIndex()
{
    return shadowPush.cascadeIndex;
}
