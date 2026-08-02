// M9: 布の描画(判断54)。両面ランバート+方向光の多段シャドウの最小シェーダー。
// 束縛するのはビューとパスのセット(set0のビュー定数と多段影定数)と照明問い合わせのセット(set3の直接光と影)だけであり、
// ジオメトリのセットも材質のセットも読まない。頂点レイアウトはscene.slangと同じ48バイト。
// 直接光と定数間接近似はscene.slangと同じ`lighting_query.slang`の宣言から読む。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import cascade_shadow;
import cascade_shadow_uniform;
import direct_light_shading;
import lighting_query;
import relative_anchor;
import view_uniform;

static const float3 clothColor = float3(0.55, 0.1, 0.12);

// 頂点入力は3属性(位置・法線・uv)。接線は消費しないためパイプライン側も宣言しない(頂点属性選択::布用)。
struct VertexInput
{
    [[vk::location(0)]] float3 position : POSITION;
    [[vk::location(1)]] float3 normal : NORMAL;
    [[vk::location(2)]] float2 uv : TEXCOORD0;
};

struct VertexOutput
{
    float4 position : SV_Position;
    float3 relativePosition : TEXCOORD1;
    float3 normal : TEXCOORD2;
    float2 uv : TEXCOORD0;
};

[shader("vertex")]
VertexOutput vertexMain(VertexInput input)
{
    VertexOutput output;
    float3 relativePosition = applyRelativeAnchor(input.position);
    output.position = mul(viewUniform.viewProjection, float4(relativePosition, 1.0));
    output.relativePosition = relativePosition;
    output.normal = input.normal;
    output.uv = input.uv;
    return output;
}

// 両面ランバート: 裏面から見た場合は法線を反転して照らす(布は薄い面のため)。
// uvからごく薄い格子模様を付けて織り目風にする(uv消費の実用も兼ねる)。
// 直接光は方向光の列を件数ぶん足す。局所光は布の見た目に要らないため読まない。
[shader("fragment")]
float4 fragmentMain(VertexOutput input, bool isFrontFace: SV_IsFrontFace) : SV_Target
{
    float3 normal = normalize(input.normal) * (isFrontFace ? 1.0 : -1.0);
    float viewDepth = viewDepthOf(input.relativePosition, viewUniform.cameraRelativePosition.xyz, viewUniform.cameraForward.xyz);
    float3 direct = float3(0.0, 0.0, 0.0);
    for (uint directionalIndex = 0; directionalIndex < lightingQueryHeader.directionalLightCount; directionalIndex++)
    {
        DirectionalLightRecord record = directionalLightRecords[directionalIndex];
        // レコードの方向は正規化済みだが、正規化を残すのは段5が絵を1画素も変えない移行だからである。
        float3 toLight = normalize(record.directionToLightAndIntensity.xyz);
        float lambert = max(dot(normal, toLight), 0.0);
        float visibility = directionalVisibilityFactor(record, input.relativePosition, viewDepth);
        direct += record.color.rgb * record.directionToLightAndIntensity.w * lambert * visibility;
    }
    // 定数間接近似はscene.slangと同じヘッダの係数から取る。これが無いと布だけが夜の環境光に追従しない。
    float3 ambient = float3(lightingQueryHeader.constantIndirectFactor, lightingQueryHeader.constantIndirectFactor, lightingQueryHeader.constantIndirectFactor);
    float weave = 0.94 + 0.06 * abs(sin(input.uv.x * 100.0) * sin(input.uv.y * 100.0));
    return float4(clothColor * weave * (direct + ambient), 1.0);
}
