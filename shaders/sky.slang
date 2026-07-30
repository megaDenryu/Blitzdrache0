// 空パスのHosek-Wilkie腕: 解析近似のHDR空を全画面三角形で描く。係数はCPU側(blitz_engineの空放射輝度係数)が焼き、ここは方向ごとの放射輝度式F(theta, gamma)を評価するだけである。
// 入力の並びと視線の復元は`sky_frame`が持つ。深度がクリア値のままの画素だけへ描く限定はパイプラインの深度比較(EQUAL、書き込みなし)が担う。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。
// 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「Hosek-Wilkie(暫定の腕。第7段完了まで既定)」

import sky_frame;

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
    float cosGamma = clamp(dot(direction, frameUniform.sunDirectionAndObserverAltitude.xyz), -1.0, 1.0);
    float gamma = acos(cosGamma);

    float daylight = frameUniform.skyRadianceScaleAndDaylight.w;
    float3 night = frameUniform.nightSkyRadianceAndSunCos.rgb;
    float3 radiance = lerp(night, skyRadiance(cosTheta, cosGamma, gamma), daylight);

    // 太陽円盤。夜は昼係数で消す(この腕は太陽が地平線より下の領域を持たないため、大気数学による遮蔽を使えない)。
    float disk = sunDiskCoverage(cosGamma) * daylight;
    radiance += frameUniform.sunDiskRadianceAndSunIrradiance.rgb * disk;

    return float4(radiance, 1.0);
}
