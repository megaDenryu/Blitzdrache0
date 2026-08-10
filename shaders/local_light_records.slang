// 局所光とクラスタの内側の値の形。局所光レコードと、セルが持つクラスタ光添字列の区間である。
// 材質のシェーダーはこのモジュールを取り込まない。取り込む側は、束縛を宣言する`local_light_binding.slang`と、
// 選別のコンピュート(`cluster_light_assignment.slang`)の2つだけである。
//
// 照明問い合わせヘッダと方向光レコード(`lighting_query_records.slang`)から分けているのは、あちらが材質の
// シェーダーが正当に読む値であり、こちらが「光の並べ方とクラスタの生成方法」そのものだからである。同じ
// ファイルへ置くと、ヘッダを読むために取り込んだ材質のシェーダーがレコードの生の形まで名指しできてしまう
// (参照: `_doc/設計/クラスタ多光源と点光源の影.md`の先行裁定「材質シェーダーは光源の並びとクラスタの生成方法を知らない」)。
//
// CPU側の正本は`crates/blitz_render/src/vulkan/lighting_query/`であり、並びの一致を単体テストが見る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct LocalLightRecord
{
    // xyzがカメラ相対位置。wは未使用である。
    float4 cameraRelativePosition;
    // rgbがscene-linearの色、wが強度。
    float4 colorAndIntensity;
    // xが光種。yzwは未使用である。
    uint4 kind;
    // 光が届く範囲の上限(メートル)。この距離で減衰の窓が0になる。
    float influenceRadius;
    // 影を持つかどうか。0が影なしであり、番兵の添字で「影なし」を表さない。
    uint hasShadow;
    // 影を持つときに見る影資源の添字。影を持たない光では読まない。
    uint shadowResourceIndex;
    // 未使用である。減衰の窓の指数は固定であり、変えたくなったときがレコードの版を上げるときである。
    uint unusedLocalLightSlot;
};

// クラスタ格子の1つのセルが持つ、クラスタ光添字列の中の区間。選別のコンピュートが書き、局所光の問い合わせが読む。
struct ClusterCell
{
    // そのセルが使うクラスタ光添字列の開始位置。
    uint start;
    // そのセルへ割り当てられた局所光の件数。
    uint count;
};

// 光種。現行は点光だけである。
static const uint localLightKindPoint = 0;
