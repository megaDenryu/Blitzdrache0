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
    // 方向も位置も問わない旧来の定数間接近似の係数。順3で放射輝度問い合わせ資源へ置き換える。
    float constantIndirectFactor;
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
