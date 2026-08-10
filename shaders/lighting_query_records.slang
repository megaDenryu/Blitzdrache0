// 材質のシェーダーが正当に読む照明の値の形。照明問い合わせヘッダと方向光レコードである。束縛先を1つも
// 宣言しないため、照明問い合わせのセットを読む画素段も、生成側のセットへ同じバッファを結ぶ選別のコンピュートも、
// 同じこの1つから形を読む。
// 束縛の宣言(`lighting_query.slang`)と分けるのは、コンピュートがset3を1つも束縛しないためである。
// 形と束縛を1つのファイルへ置くと、コンピュートが取り込むだけで画素段専用の束縛先を宣言したことになる。
// 局所光レコードとクラスタの区間は`local_light_records.slang`が別に持つ。材質のシェーダーがヘッダを読むために
// 取り込んだだけで、光の並べ方とクラスタの生成方法まで名指しできることを避けるためである。
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

// 可視性の種別。番兵の添字で「影なし」を表さず、種別そのもので枝を分ける。
static const uint directionalVisibilityNone = 0;
static const uint directionalVisibilityCascadedShadow = 1;
