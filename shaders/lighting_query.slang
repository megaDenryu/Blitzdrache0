// 照明問い合わせのセット(set3)の唯一の宣言。シーン描画と布描画がこの1つのモジュールを取り込み、
// ヘッダ・方向光レコード列・局所光レコード列・多段シャドウマップを同じ形で読む。
// 直接光は最初から配列契約であり、現行の方向光1件と点光源1件も件数1の列として並ぶ
// (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「順5(多光源クラスタード)への接続」)。
// CPU側の正本は`crates/blitz_render/src/vulkan/lighting_query/`であり、並びの一致を単体テストが見る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct LightingQueryHeader
{
    uint directionalLightCount;
    uint localLightCount;
    // 0のときシーンの画素段はベースカラーをそのまま返す。件数0とは別の状態である。
    uint lightingEnabled;
    // 方向も位置も問わない定数間接近似の係数。定数近似の契約のパイプラインだけが読む。
    float constantIndirectFactor;
    // xyzがワールドの太陽方向(表面から太陽へ向かう単位ベクトル)、wは未使用である。
    // 遠方環境の契約のパイプラインだけが読み、法線と反射方向を太陽相対座標へ写すのに使う。
    float4 sunDirection;
    // 以下の3区画はクラスタ写像のパラメータであり、画素段と選別のコンピュートが同じ値を読む。
    // 読み手が現れるのはクラスタの選別を入れる段であり、この版では書かれるだけである。
    // xyzがクラスタ格子の横・縦・奥行きの分割数、wは未使用である。
    uint4 clusterGridDivisions;
    // xyがクラスタ写像が見る画面の横・縦の画素数、zwは未使用である。
    uint4 clusterScreenPixelExtent;
    // xがビュー空間の奥行きの近、yが遠、zwが視野の半角の正接の横と縦である。
    float4 clusterDepthRangeAndHalfAngleTangent;
};

struct DirectionalLightRecord
{
    // xyzが表面から光へ向かう正規化方向、wが強度。
    float4 directionToLightAndIntensity;
    // rgbがscene-linearの色。wは未使用である。
    float4 color;
    // xが可視性の種別、yが多段影のときの影資源添字。zwは未使用である。
    uint4 visibility;
};

struct LocalLightRecord
{
    // xyzがカメラ相対位置。wは未使用である。
    float4 cameraRelativePosition;
    // rgbがscene-linearの色、wが強度。
    float4 colorAndIntensity;
    // xが光種。yzwは未使用である。
    uint4 kind;
    // 光が届く範囲の上限(メートル)。減衰の窓を掛ける段まで読み手は居ない。
    float influenceRadius;
    // 影を持つかどうか。0が影なしであり、番兵の添字で「影なし」を表さない。
    uint hasShadow;
    // 影を持つときに見る影資源の添字。影を持たない光では読まない。
    uint shadowResourceIndex;
    // 未使用である。減衰の窓の指数は固定であり、変えたくなったときがレコードの版を上げるときである。
    uint unusedLocalLightSlot;
};

// 可視性の種別。番兵の添字で「影なし」を表さず、種別そのもので枝を分ける。
static const uint directionalVisibilityNone = 0;
static const uint directionalVisibilityCascadedShadow = 1;
// 光種。現行は点光だけである。
static const uint localLightKindPoint = 0;

// 多段シャドウマップ(判断35・OW5第5段)。距離区分ごとの層を持つ2D配列テクスチャ+比較サンプラーを同一bindingへ束ねる。
[[vk::binding(0, 3)]] [[vk::combinedImageSampler]] Texture2DArray shadowMapTexture;
[[vk::binding(0, 3)]] [[vk::combinedImageSampler]] SamplerComparisonState shadowMapSampler;
[[vk::binding(1, 3)]] ConstantBuffer<LightingQueryHeader> lightingQueryHeader;
[[vk::binding(2, 3)]] StructuredBuffer<DirectionalLightRecord> directionalLightRecords;
[[vk::binding(3, 3)]] StructuredBuffer<LocalLightRecord> localLightRecords;
