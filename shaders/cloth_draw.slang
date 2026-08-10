// M9: 布の描画(判断54)。定数近似の契約の両面ランバート+方向光の多段シャドウ+局所光の最小シェーダー。
// 束縛するのはビューとパスのセット(set0のビュー定数と多段影定数)と照明問い合わせのセット(set3の直接光と影)だけであり、
// ジオメトリのセットも材質のセットも読まない。頂点レイアウトはscene.slangと同じ48バイト。
// 直接光と定数間接近似はscene.slangと同じ`lighting_query.slang`の宣言から読む。
// 頂点段はこの1本を両方の契約が共有する。遠方環境の契約の画素段は`cloth_draw_distant_environment.slang`にあり、
// 供給元に依らない部分は`cloth_shading.slang`を両方が取り込む。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import cloth_io;
import cloth_shading;
import lighting_query;
import motion_vector;
import relative_anchor;
import view_uniform;

[shader("vertex")]
VertexOutput vertexMain(VertexInput input)
{
    VertexOutput output;
    float3 relativePosition = applyRelativeAnchor(input.position);
    output.position = mul(viewUniform.viewProjection, float4(relativePosition, 1.0));
    output.currentClipPosition = output.position;
    // 布の変形を含まず、カメラの運動だけを表す前フレームの位置である。前フレームの頂点位置を保つ形は第2弾で入る。
    output.previousClipPosition = mul(viewUniform.previousViewProjection, float4(relativePosition, 1.0));
    output.relativePosition = relativePosition;
    output.normal = input.normal;
    output.uv = input.uv;
    return output;
}

[shader("fragment")]
ColorAndMotionOutput fragmentMain(VertexOutput input, bool isFrontFace: SV_IsFrontFace)
{
    float3 normal = clothSurfaceNormal(input.normal, isFrontFace);
    float3 direct = clothDirectLight(normal, input.relativePosition, uint2(input.position.xy));
    // 定数間接近似はscene.slangと同じヘッダの係数から取る。これが無いと布だけが夜の環境光に追従しない。
    float3 ambient = float3(lightingQueryHeader.constantIndirectFactor, lightingQueryHeader.constantIndirectFactor, lightingQueryHeader.constantIndirectFactor);
    return clothColorAndMotion(clothColor * clothWeave(input.uv) * (direct + ambient), input);
}
