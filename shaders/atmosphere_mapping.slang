// 透過率LUTのテクセル座標と、観測点の半径・視線の天頂余弦との行き来。
// 注意: この式は`crates/blitz_render/src/atmosphere/mapping/transmittance_mapping.rs`の写しである。
// 正本はRust側にあり、写像と逆写像の往復・軸の意味のユニットテストもそちらが持つ。
// ここが正本から外れると、焼いた座標と引く座標が食い違って空の色が静かにずれる。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_medium;

struct AtmosphereRay
{
    float radius;
    float zenithCosine;
};

// 観測点の高さで到達しうる地平線までの弦の長さ。縦軸の目盛りの分母になる。
float horizonChordOf(AtmosphereMedium medium)
{
    float bottom = bottomRadiusOf(medium);
    float top = topRadiusOf(medium);
    return sqrt(max(0.0, top * top - bottom * bottom));
}

// テクセル中心のUVから視線を求める。縦が観測点の高さ、横が大気上端までの距離の位置である。
AtmosphereRay rayFromTransmittanceUv(AtmosphereMedium medium, float2 uv)
{
    float bottom = bottomRadiusOf(medium);
    float top = topRadiusOf(medium);
    float horizonChord = horizonChordOf(medium);
    float groundChord = horizonChord * uv.y;
    float radius = clamp(sqrt(groundChord * groundChord + bottom * bottom), bottom, top);
    float shortest = top - radius;
    float longest = groundChord + horizonChord;
    float distance = shortest + uv.x * (longest - shortest);

    AtmosphereRay ray;
    ray.radius = radius;
    ray.zenithCosine = distance <= 0.0
        ? 1.0
        : clamp((horizonChord * horizonChord - groundChord * groundChord - distance * distance) / (2.0 * radius * distance), -1.0, 1.0);
    return ray;
}

// 視線からテクセル中心のUVを求める。`rayFromTransmittanceUv`の逆写像である。
float2 transmittanceUvFromRay(AtmosphereMedium medium, AtmosphereRay ray)
{
    float bottom = bottomRadiusOf(medium);
    float top = topRadiusOf(medium);
    float horizonChord = horizonChordOf(medium);
    float groundChord = sqrt(max(0.0, ray.radius * ray.radius - bottom * bottom));
    float distance = distanceToTop(medium, ray.radius, ray.zenithCosine);
    float shortest = top - ray.radius;
    float longest = groundChord + horizonChord;
    float horizontal = longest <= shortest ? 0.0 : (distance - shortest) / (longest - shortest);
    float vertical = horizonChord <= 0.0 ? 0.0 : groundChord / horizonChord;
    return float2(horizontal, vertical);
}

// 書き込み先のテクセルの添字から、そのテクセルの中心のUVを求める。書き込み側も読み出し側も中心を標本点にする。
float2 texelCenterUv(uint2 texel, uint2 resolution)
{
    return (float2(texel) + 0.5) / float2(resolution);
}
