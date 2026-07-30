// 空パスの大気LUT腕: 視線の向きをHillaire写像でスカイビューLUTのUVへ写し、標本した放射輝度へ太陽の放射照度を掛ける。
// LUTは放射照度1で焼いてあるため、尺度を掛けるのは描画側の役目である。視線が地面と交差する向きはLUTが地表の
// 照り返しを含んでいるため、そのまま使う。夜間背景放射輝度は昼夜の切替なしに常時加算する。
// 太陽円盤はLUTに含まれないため、円盤の放射輝度へ視線上の大気透過率と惑星による遮蔽を掛けた別項として足す。
// LUTの引き方は`sky_atmosphere_lookup`が、入力の並びと視線の復元は`sky_frame`が持つ。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。
// 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「大気LUT方式の設計(第7段で実装する)」

import atmosphere_medium;
import atmosphere_skyview_mapping;
import sky_atmosphere_lookup;
import sky_frame;

// 惑星による直射の遮蔽。太陽の角半径ぶんの幅で滑らかに落とすのは、円盤が地平線をまたぐ間は一部だけが隠れるためである。
float sunVisibilityAboveHorizon(float horizonZenithCosine, float sunZenithCosine)
{
    float sunAngularRadius = acos(clamp(frameUniform.nightSkyRadianceAndSunCos.w, -1.0, 1.0));
    float horizonAngle = acos(clamp(horizonZenithCosine, -1.0, 1.0));
    float sunZenithAngle = acos(clamp(sunZenithCosine, -1.0, 1.0));
    return 1.0 - smoothstep(horizonAngle - sunAngularRadius, horizonAngle + sunAngularRadius, sunZenithAngle);
}

[shader("fragment")]
float4 fragmentMain(VertexOutput input) : SV_Target
{
    AtmosphereMedium medium = atmosphereMedium;
    float3 direction = normalize(input.viewRay);
    float3 sunDirection = frameUniform.sunDirectionAndObserverAltitude.xyz;
    float observerAltitude = frameUniform.sunDirectionAndObserverAltitude.w;
    // 観測半径を上端で切り詰めるのは、写像の平方根の中身が非負である根拠が「観測点が大気の内側にある」ことだからである。
    float observerRadius = min(bottomRadiusOf(medium) + observerAltitude, topRadiusOf(medium));

    SkyViewRay ray = skyViewRayOfWorldDirection(direction, sunDirection);
    float sunIrradiance = frameUniform.sunDiskRadianceAndSunIrradiance.w;
    float3 radiance = sampleSkyView(medium, observerRadius, ray) * sunIrradiance;
    radiance += frameUniform.nightSkyRadianceAndSunCos.rgb;

    float sunZenithCosine = clamp(sunDirection.y, -1.0, 1.0);
    float horizonZenithCosine = skyHorizonZenithCosine(bottomRadiusOf(medium), observerRadius);
    float disk = sunDiskCoverage(clamp(dot(direction, sunDirection), -1.0, 1.0));
    float visibility = sunVisibilityAboveHorizon(horizonZenithCosine, sunZenithCosine);
    float3 diskTransmittance = sunTransmittanceAtObserver(medium, observerRadius, sunZenithCosine);
    radiance += frameUniform.sunDiskRadianceAndSunIrradiance.rgb * (disk * visibility) * diskTransmittance;

    return float4(max(radiance, 0.0), 1.0);
}
