// 照明問い合わせのセット(set3)の束縛先の唯一の宣言。シーン描画と布描画がこの1つのモジュールを取り込み、
// ヘッダ・方向光レコード列・局所光レコード列・多段シャドウマップ・クラスタ格子・クラスタ光添字列を同じ形で読む。
// 値の形そのものは`lighting_query_records.slang`が持つ。分けているのは、選別のコンピュートがset3を1つも
// 束縛せずに同じ形を読むためである。
// 直接光は最初から配列契約であり、方向光と局所光はどちらも件数ぶんの列として並ぶ
// (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「順5(多光源クラスタード)への接続」)。
// クラスタ格子とクラスタ光添字列は照明束縛レイアウトの両方の枝が等しく宣言する。局所光が0件の世界も
// 件数0のセルだけを持つ格子を読み、枝もパイプラインキーも増やさない
// (参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断f」)。
// CPU側の正本は`crates/blitz_render/src/vulkan/lighting_query/`であり、並びの一致を単体テストが見る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

__exported import lighting_query_records;

// 多段シャドウマップ(判断35・OW5第5段)。距離区分ごとの層を持つ2D配列テクスチャ+比較サンプラーを同一bindingへ束ねる。
[[vk::binding(0, 3)]] [[vk::combinedImageSampler]] Texture2DArray shadowMapTexture;
[[vk::binding(0, 3)]] [[vk::combinedImageSampler]] SamplerComparisonState shadowMapSampler;
[[vk::binding(1, 3)]] ConstantBuffer<LightingQueryHeader> lightingQueryHeader;
[[vk::binding(2, 3)]] StructuredBuffer<DirectionalLightRecord> directionalLightRecords;
[[vk::binding(3, 3)]] StructuredBuffer<LocalLightRecord> localLightRecords;
// セルごとの区間表。添字は横が最も速く変わり、次に縦、最後に奥行きである(CPU正本の`cell_index`と同じ規約)。
[[vk::binding(8, 3)]] StructuredBuffer<ClusterCell> clusterCells;
// セルの区間が指す局所光レコードの添字の列。
[[vk::binding(9, 3)]] StructuredBuffer<uint> clusterLightIndices;
