// 点光源の影の立方体配列を比較サンプリングする工程。受け取るのは灯から表面へ向かうベクトルと影響半径と
// 立方体の番号、返すのは影係数(1が照らされる、0が遮られる)である。
// 束縛先そのものはこのモジュールが持たない。呼び出し側が`local_light_binding`の2つを引数で渡すのは、
// 多段影の`shadow_sample`が同じ形を採っているのと同じ理由であり、この工程がどのセットの何番かを知らずに済む。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import point_light_shadow_projection;

// 比較する主軸の距離から引く、世界の長さの深度の偏り(メートル)。CPU正本の`点光源の影の深度の偏りのメートル`と
// 同じ値でなければならない。
static const float pointLightShadowDepthBiasMeters = 0.05;

// 灯から表面へ向かう向きで立方体を1度参照する。比較サンプラーが線形フィルタで4テクセルを混ぜるため、
// 面の境界をまたぐ補間もハードウェアの側で働く(平面へ詰める集約画像を採らなかった理由がこれである)。
//
// 深度の偏りは主軸の距離から引く形で掛ける。比較深度を作ってから引くのではなく距離の側で引くのは、
// 透視投影の深度が近面の近くへ寄っており、同じ深度の差が世界の長さでは距離によって別物になるためである。
float pointLightShadowFactor(
    float3 lightToSurface, float influenceRadius, uint shadowResourceIndex,
    TextureCubeArray shadowTexture, SamplerComparisonState shadowSampler)
{
    float majorAxisDistance = pointLightShadowMajorAxisDistance(lightToSurface);
    float biasedDistance = max(majorAxisDistance - pointLightShadowDepthBiasMeters, pointLightShadowNearPlaneMeters);
    float comparisonDepth = pointLightShadowComparisonDepth(biasedDistance, influenceRadius);
    // 影響半径の外は記録の外である。窓が0へ落ちて寄与が消える範囲のため、影なしで返して分岐を1つ減らす。
    if (comparisonDepth > 1.0)
    {
        return 1.0;
    }
    float4 coordinate = float4(lightToSurface, float(shadowResourceIndex));
    return shadowTexture.SampleCmp(shadowSampler, coordinate, comparisonDepth);
}
