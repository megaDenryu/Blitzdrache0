// 多段影定数の唯一の宣言。シーンの影評価とシャドウ記録パスがこの1つを共有する。
// シャドウ記録の頂点段が「ビュー定数の先頭を別の構造体として読む」バイト配置の暗黙契約はこのモジュールの導入で廃止した
// (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「分離の形」)。
// CPU側の正本は`crates/blitz_render/src/vulkan/uniform/cascade_bytes.rs`であり、並びの一致を単体テストが見る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct CascadeShadowUniform
{
    // column_major明示(CPU側は列優先配列)。距離区分の数だけ光源ビュー射影を持つ。
    column_major float4x4 cascadeLightViewProjection[4];
    // xyzが距離区分の境界のビュー空間深度。wは未使用である。
    float4 cascadeSplitDepths;
    // xyzが距離区分の境界の手前に置く遷移域の幅。wは未使用である。
    float4 cascadeTransitionWidths;
    // xがシーンの画素段が出す診断の種別(0は出さない、1は距離区分の可視化、2は影の欠落計器)、yがシャドウマップのテクセル尺度。zwは未使用である。
    // 方向光が影を落とすかはここに無い。遮蔽の判定方法は光ごとに違うため、照明問い合わせの方向光レコードの可視性が持つ。
    float4 cascadeDebugAndTexelSize;
};

[[vk::binding(1, 0)]]
ConstantBuffer<CascadeShadowUniform> cascadeShadowUniform;
