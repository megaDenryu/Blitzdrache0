// 天頂を+Yに取った向きと太陽方向から、スカイビューのベイク済み画像の縦横が意味する視線を求める工程。
// ディスクリプタを1つも宣言しないのは、この写像を空背景パス(画素段)と遠方環境生成(コンピュート)の両方が使い、
// 束縛の形が消費者ごとに違うためである。
// 注意: この写像は`crates/blitz_render/src/atmosphere/mapping/skyview_lookup.rs`の写しである。正本はRust側にある。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_skyview_mapping;

// 水平成分がこの長さ未満なら方位を決められないとみなす。天頂と天底の行はどの横位置でも同じ値であり、
// 天頂の太陽はどの方位から見ても対称であるため、この場合にどの方位を選んでも参照する値は変わらない。
// 測るのは`length`(ユークリッドの長さ)であり、正本の`方位を決められない水平成分の長さ`と同じノルムである。
static const float skyAzimuthEpsilon = 1.0e-6;

// 方位余弦を内積そのものでなく水平面へ落としてから取るのは、横軸が太陽との相対方位であって角度ではないためである。
SkyViewRay skyViewRayOfWorldDirection(float3 viewDirection, float3 sunDirection)
{
    float2 viewHorizontal = float2(viewDirection.x, viewDirection.z);
    float2 sunHorizontal = float2(sunDirection.x, sunDirection.z);
    float azimuthCosine = 1.0;
    if (length(viewHorizontal) >= skyAzimuthEpsilon && length(sunHorizontal) >= skyAzimuthEpsilon)
    {
        azimuthCosine = clamp(dot(normalize(viewHorizontal), normalize(sunHorizontal)), -1.0, 1.0);
    }

    SkyViewRay ray;
    ray.viewZenithCosine = clamp(viewDirection.y, -1.0, 1.0);
    ray.sunAzimuthCosine = azimuthCosine;
    return ray;
}
