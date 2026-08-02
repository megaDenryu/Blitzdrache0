// ビュー・シーンパス定数の唯一の宣言。全シーン描画が同じビューで読む小値(ビュー射影・カメラ相対位置・カメラ前方向)と、
// 直接照明の値をここが持つ。読む側は自分で構造体を書かずにこのモジュールを取り込む。
// 直接照明をここへ置くのは仮置きであり、照明問い合わせセットの導入で移す
// (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「分離の形」)。
// CPU側の正本は`crates/blitz_render/src/vulkan/uniform/view_pass_bytes.rs`であり、並びの一致を単体テストが見る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct ViewPassUniform
{
    // column_major明示(CPU側は列優先配列)。
    column_major float4x4 viewProjection;
    float4 cameraRelativePosition;
    // xyzがカメラの前方向。ビュー空間深度を内積で求めるために渡す。wは未使用である。
    float4 cameraForward;
    float4 directionalLightDirection;
    // rgbが方向光の色、wが強度。
    float4 directionalLightColor;
    float4 pointLightPosition;
    // rgbが点光源の色、wが強度。
    float4 pointLightColor;
    // xが環境光係数、yがライティング有効(0か1)。zとwは未使用である。
    float4 ambientAndLightingEnabled;
};

[[vk::binding(0, 0)]]
ConstantBuffer<ViewPassUniform> viewPassUniform;
