// 空パス(OW5第2段): Hosek-Wilkie解析近似のHDR空を全画面三角形で描く。係数はCPU側(blitz_engineの空放射輝度係数)が焼き、ここは方向ごとの放射輝度式F(theta, gamma)を評価するだけである。
// 深度がクリア値のままの画素だけへ描く限定はパイプラインの深度比較(EQUAL、書き込みなし)が担う。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。
// 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「空の方式」

// binding3のフレームユニフォーム。前半はscene.slangのFrameUniformと同じ並びであり、後半の inverseViewProjection 以降が空パスだけが読む区画である。
// 注意: 前半の並びをscene.slangと食い違わせると後半のオフセットがずれる。
struct SkyFrameUniform
{
    column_major float4x4 cascadeLightViewProjection[4];
    column_major float4x4 viewProjection;
    float4 cameraRelativePosition;
    float4 directionalLightDirection;
    float4 directionalLightColor;
    float4 pointLightPosition;
    float4 pointLightColor;
    float4 ambientAndLighting;
    float4 cascadeSplitDepths;
    float4 cascadeTransitionWidths;
    float4 cameraForward;
    column_major float4x4 inverseViewProjection;
    // rgbが係数A〜I、wは未使用。
    float4 skyCoefficients[9];
    // rgbが放射輝度スケール、wが昼係数。
    float4 skyRadianceScaleAndDaylight;
    // rgbが夜空の最小項、wが太陽円盤の角半径の余弦。
    float4 nightSkyRadianceAndSunCos;
    // xyzが地表から太陽へ向かう単位ベクトル(Xが東・Yが天頂・Zが南)。
    float4 sunDirection;
    float4 sunDiskRadiance;
};

[[vk::binding(3, 0)]]
ConstantBuffer<SkyFrameUniform> frameUniform;

struct VertexOutput
{
    float4 position : SV_Position;
    // カメラ相対フレームでの視線ベクトル。全画面三角形はwが一定のため、頂点間の線形補間が正確な向きを与える。
    float3 viewRay : TEXCOORD0;
};

[shader("vertex")]
VertexOutput vertexMain(uint vertexID : SV_VertexID)
{
    // 全画面三角形: 頂点ID 0,1,2 をクリップ座標 (-1,-1),(3,-1),(-1,3) に写す。
    // 深度は遠平面ちょうど(z=w=1)に置く。深度クリア値と同じ値であり、パイプラインのEQUAL比較が未描画画素だけを選ぶ。
    float2 corner = float2(float((vertexID << 1) & 2), float(vertexID & 2));
    float2 ndc = corner * 2.0 - 1.0;
    VertexOutput output;
    output.position = float4(ndc, 1.0, 1.0);
    float4 farPoint = mul(frameUniform.inverseViewProjection, float4(ndc, 1.0, 1.0));
    output.viewRay = farPoint.xyz / farPoint.w - frameUniform.cameraRelativePosition.xyz;
    return output;
}

// 分母が0にならないよう天頂方向の余弦へ足す下駄(原実装と同じ値)。
static const float COSINE_BIAS = 0.01;

// Hosek-Wilkieの放射輝度式。cosTheta は見る向きの天頂余弦、gamma は太陽との角度である。
float3 skyRadiance(float cosTheta, float cosGamma, float gamma)
{
    float3 coefficient[9];
    for (uint index = 0; index < 9; ++index)
    {
        coefficient[index] = frameUniform.skyCoefficients[index].rgb;
    }
    float3 expTerm = exp(coefficient[4] * gamma);
    float rayleighTerm = cosGamma * cosGamma;
    float3 mieDenominator = max(1.0 + coefficient[8] * coefficient[8] - 2.0 * coefficient[8] * cosGamma, 1e-6);
    float3 mieTerm = (1.0 + cosGamma * cosGamma) / pow(mieDenominator, 1.5);
    float zenithTerm = sqrt(cosTheta);
    float3 horizonGain = 1.0 + coefficient[0] * exp(coefficient[1] / (cosTheta + COSINE_BIAS));
    float3 directionTerm = coefficient[2] + coefficient[3] * expTerm + coefficient[5] * rayleighTerm
        + coefficient[6] * mieTerm + coefficient[7] * zenithTerm;
    return max(horizonGain * directionTerm * frameUniform.skyRadianceScaleAndDaylight.rgb, 0.0);
}

[shader("fragment")]
float4 fragmentMain(VertexOutput input) : SV_Target
{
    float3 direction = normalize(input.viewRay);
    // 注意: 天頂余弦を0で頭打ちにするのは、地平線より下の向きが解析近似の定義域外だからである。外挿せず地平線の値を保つ。
    float cosTheta = clamp(direction.y, 0.0, 1.0);
    float cosGamma = clamp(dot(direction, frameUniform.sunDirection.xyz), -1.0, 1.0);
    float gamma = acos(cosGamma);

    float daylight = frameUniform.skyRadianceScaleAndDaylight.w;
    float3 night = frameUniform.nightSkyRadianceAndSunCos.rgb;
    float3 radiance = lerp(night, skyRadiance(cosTheta, cosGamma, gamma), daylight);

    // 太陽円盤。角半径の内外を余弦で比べ、縁を1割の幅で滑らかにしてジャギーを避ける。
    float sunCos = frameUniform.nightSkyRadianceAndSunCos.w;
    float outerCos = cos(acos(sunCos) * 1.1);
    float innerCos = cos(acos(sunCos) * 0.9);
    float disk = smoothstep(outerCos, innerCos, cosGamma) * daylight;
    radiance += frameUniform.sunDiskRadiance.rgb * disk;

    return float4(radiance, 1.0);
}
