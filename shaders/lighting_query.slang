// 照明問い合わせのセット(set3)のうち、材質のシェーダーが正当に読む束縛先の唯一の宣言。多段シャドウマップ・
// ヘッダ・方向光レコード列の3つである。シーン描画と布描画がこの1つのモジュールを取り込む。
// 値の形そのものは`lighting_query_records.slang`が持つ。分けているのは、選別のコンピュートがset3を1つも
// 束縛せずに同じ形を読むためである。
//
// 局所光レコード列とクラスタの2本(binding3・8・9)はここに無い。`local_light_binding.slang`が持ち、材質の
// シェーダーはそちらを取り込まない。方向光をここへ残すのは、方向光を全画素段が別に列挙するという先行裁定に
// よるものであり、材質のシェーダーによる正当な消費だからである
// (参照: `_doc/設計/クラスタ多光源と点光源の影.md`の先行裁定と判断f)。
// CPU側の正本は`crates/blitz_render/src/vulkan/lighting_query/`であり、並びの一致を単体テストが見る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

__exported import lighting_query_records;

// 多段シャドウマップ(判断35・OW5第5段)。距離区分ごとの層を持つ2D配列テクスチャ+比較サンプラーを同一bindingへ束ねる。
[[vk::binding(0, 3)]] [[vk::combinedImageSampler]] Texture2DArray shadowMapTexture;
[[vk::binding(0, 3)]] [[vk::combinedImageSampler]] SamplerComparisonState shadowMapSampler;
[[vk::binding(1, 3)]] ConstantBuffer<LightingQueryHeader> lightingQueryHeader;
[[vk::binding(2, 3)]] StructuredBuffer<DirectionalLightRecord> directionalLightRecords;
