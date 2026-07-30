// 空パスの大気のベイク済み画像方式: 視線の向きをHillaire写像でスカイビューのベイク済み画像のUVへ写し、標本した放射輝度へ太陽の放射照度を掛ける。
// ベイク済み画像は放射照度1で焼いてあるため、尺度を掛けるのは描画側の役目である。視線が地面と交差する向きはベイク済み画像が地表の
// 照り返しを含んでいるため、そのまま使う。夜間背景放射輝度は昼夜の切替なしに常時加算する。
// 太陽円盤はベイク済み画像に含まれないため、円盤の放射輝度へ視線上の大気透過率と惑星による遮蔽を掛けた別項として足す。
// 透過率と遮蔽はどちらも円盤の中心方向でなく画素ごとの視線で評価する。円盤は有限の角半径を持つため、
// 地平線をまたぐ間は上側の画素が大気上端へ抜けるのに下側の画素が惑星へ入る。中心方向で1つの値を作ると
// 円盤全体が同じ明るさになり、欠けも中心角の補間でしか表せない。画素ごとに判定すれば下側だけが欠ける。
// 正本は`crates/blitz_render/src/atmosphere/integration/sun_visibility.rs`である。
// ベイク済み画像の参照の仕方は`sky_atmosphere_lookup`が、入力の並びと視線の復元は`sky_frame`が持つ。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。
// 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「大気のベイク済み画像方式の設計(第7段で実装する)」

import atmosphere_medium;
import atmosphere_skyview_mapping;
import sky_atmosphere_lookup;
import sky_frame;

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

    // 惑星の縁は幾何としての硬い境界であるため、可視は画素ごとの真偽で切る。地平線より下を向く視線は
    // 惑星と交わるため直射が届かない。境界の等号は「交わらない」側にあり、正本の`視線が惑星と交差するか`
    // (水平の視線を交わらないと数える)に揃える。
    float viewZenithCosine = clamp(direction.y, -1.0, 1.0);
    float horizonZenithCosine = skyHorizonZenithCosine(bottomRadiusOf(medium), observerRadius);
    float disk = sunDiskCoverage(clamp(dot(direction, sunDirection), -1.0, 1.0));
    float visibility = viewZenithCosine >= horizonZenithCosine ? 1.0 : 0.0;
    float3 diskTransmittance = sunTransmittanceAlongView(medium, observerRadius, viewZenithCosine);
    radiance += frameUniform.sunDiskRadianceAndSunIrradiance.rgb * (disk * visibility) * diskTransmittance;

    return float4(max(radiance, 0.0), 1.0);
}
