// 照明問い合わせのセット(set3)のうち、局所光とクラスタの内側だけの束縛先の唯一の宣言。局所光レコード列(binding3)・クラスタ格子の区間表(binding8)・クラスタ光添字列(binding9)である。
//
// 材質のシェーダーはこのモジュールを取り込まない。
// 取り込むのは局所光の問い合わせ(`local_light_shading.slang`)だけであり、あちらが取り込みを再輸出しないため、消費側からはこの3つの名前が見えない。
// 見えないことが「材質シェーダーは光源の並びとクラスタの生成方法を知らない」という先行裁定の実効である。
// 取り込みを1行足せないことは`cargo xtask conform`の`module_import_boundary`が見る(参照: `_doc/設計/クラスタ多光源と点光源の影.md`)。
//
// クラスタの2本は照明束縛レイアウトの両方の枝が等しく宣言する。局所光が0件の世界も件数0のセルだけを持つ格子を読み、枝もパイプラインキーも増やさない(判断f)。
// 書く側の選別のコンピュートは同じ3本を自分の生成側のセットへ結び、このモジュールを取り込まない。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

__exported import local_light_records;

[[vk::binding(3, 3)]] StructuredBuffer<LocalLightRecord> localLightRecords;
// セルごとの区間表。添字は横が最も速く変わり、次に縦、最後に奥行きである(CPU正本の`cell_index`と同じ規約)。
[[vk::binding(8, 3)]] StructuredBuffer<ClusterCell> clusterCells;
// セルの区間が指す局所光レコードの添字の列。
[[vk::binding(9, 3)]] StructuredBuffer<uint> clusterLightIndices;
