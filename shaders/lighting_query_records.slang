// 照明問い合わせが運ぶ値の形の宣言だけを持つモジュール。束縛先を1つも宣言しないため、照明問い合わせのセットを
// 読む画素段も、生成側のセットへ同じバッファを結ぶ選別のコンピュートも、同じこの1つから形を読む。
// 束縛の宣言(`lighting_query.slang`)と分けるのは、コンピュートがset3を1つも束縛しないためである。
// 形と束縛を1つのファイルへ置くと、コンピュートが取り込むだけで画素段専用の束縛先を宣言したことになる。
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
    // 光が届く範囲の上限(メートル)。この距離で減衰の窓が0になる。
    float influenceRadius;
    // 影を持つかどうか。0が影なしであり、番兵の添字で「影なし」を表さない。
    uint hasShadow;
    // 影を持つときに見る影資源の添字。影を持たない光では読まない。
    uint shadowResourceIndex;
    // 未使用である。減衰の窓の指数は固定であり、変えたくなったときがレコードの版を上げるときである。
    uint unusedLocalLightSlot;
};

// クラスタ格子の1つのセルが持つ、クラスタ光添字列の中の区間。選別のコンピュートが書き、画素段が読む。
struct ClusterCell
{
    // そのセルが使うクラスタ光添字列の開始位置。
    uint start;
    // そのセルへ割り当てられた局所光の件数。
    uint count;
};

// 可視性の種別。番兵の添字で「影なし」を表さず、種別そのもので枝を分ける。
static const uint directionalVisibilityNone = 0;
static const uint directionalVisibilityCascadedShadow = 1;
// 光種。現行は点光だけである。
static const uint localLightKindPoint = 0;
