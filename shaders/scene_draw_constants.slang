// シーン描画1回ぶんの描画定数(プッシュ定数16バイト)。カメラ相対の基準原点と、その発行が塗る材質のレコード添字を運ぶ。
// 布・粒子が読む`RelativeAnchor`の16バイトと別に持つのは、材質レコード添字を材質を持たないパイプラインへ波及させない
// ためである。シャドウが読む20バイトの定数とも別のままにする
// (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「分離の形」)。
// 注意: 頂点ステージが基準原点を、画素段ステージが材質レコード添字を読むため、パイプラインレイアウトはこの範囲を
// 両ステージへ宣言する。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct SceneDrawConstants
{
    float3 position;
    uint materialRecordIndex;
};

[[vk::push_constant]]
SceneDrawConstants sceneDrawConstants;

// 基準原点を加算してカメラ相対のワールド位置にする。
float3 applySceneDrawAnchor(float3 worldPosition)
{
    return worldPosition + sceneDrawConstants.position;
}
