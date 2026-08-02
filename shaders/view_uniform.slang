// ビュー定数の唯一の宣言。全シーン描画が同じビューで読む小値(ビュー射影・カメラ相対位置・カメラ前方向)だけを持つ。
// 読む側は自分で構造体を書かずにこのモジュールを取り込む。
// 直接照明の値はここに無い。方向光と局所光は照明問い合わせのセットのレコード列が持つ
// (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「分離の形」)。
// CPU側の正本は`crates/blitz_render/src/vulkan/uniform/view_bytes.rs`であり、並びの一致を単体テストが見る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct ViewUniform
{
    // column_major明示(CPU側は列優先配列)。
    column_major float4x4 viewProjection;
    float4 cameraRelativePosition;
    // xyzがカメラの前方向。ビュー空間深度を内積で求めるために渡す。wは未使用である。
    float4 cameraForward;
};

[[vk::binding(0, 0)]]
ConstantBuffer<ViewUniform> viewUniform;
