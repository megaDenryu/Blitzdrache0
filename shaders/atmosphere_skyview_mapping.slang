// スカイビューのベイク済み画像のテクセル座標と、視線の天頂余弦・太陽との方位余弦との行き来。
// 縦軸を地平線で2つの区間へ分け、それぞれの中で境界へ標本を寄せる。横軸は太陽方位余弦を平方根で刻む。
// 注意: この式は`crates/blitz_render/src/atmosphere/mapping/skyview_mapping.rs`と`unit_texel.rs`の写しである。
// 正本はRust側にあり、往復と軸の意味のユニットテストもそちらが持つ。ここが正本から外れると、焼いた座標と
// 参照する座標が食い違って空の色が静かにずれる。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_medium;

struct SkyViewRay
{
    float viewZenithCosine;
    float sunAzimuthCosine;
};

// 分母が0以下のときに0を返す除算。観測半径が0になるのは惑星中心に観測点を置いた場合だけであり、
// 呼び出し元の不変条件がそれを禁じているため、これは丸めで境界へ落ちた場合の備えである。
float skyViewSafeDivide(float numerator, float denominator)
{
    return denominator <= 0.0 ? 0.0 : numerator / denominator;
}

// xが天頂から地平線までの角、yが地平線から天底までの角。2つの和は必ず円周角の半分である。
float2 skyViewVerticalLayout(AtmosphereMedium medium, float observerRadius)
{
    float bottom = bottomRadiusOf(medium);
    float horizonChord = sqrt(max(0.0, observerRadius * observerRadius - bottom * bottom));
    float horizonToNadir = acos(clamp(skyViewSafeDivide(horizonChord, observerRadius), -1.0, 1.0));
    return float2(atmospherePi - horizonToNadir, horizonToNadir);
}

// 単位位置から視線を求める。縦が0.5未満なら地面と交わらない向き、0.5以上なら地面と交わる向きになる。
SkyViewRay rayFromSkyViewUnit(AtmosphereMedium medium, float observerRadius, float2 unitPosition)
{
    float2 layout = skyViewVerticalLayout(medium, observerRadius);
    float zenithAngle;
    if (unitPosition.y < 0.5)
    {
        float bias = 1.0 - 2.0 * unitPosition.y;
        zenithAngle = layout.x * (1.0 - bias * bias);
    }
    else
    {
        float bias = 2.0 * unitPosition.y - 1.0;
        zenithAngle = layout.x + layout.y * bias * bias;
    }

    SkyViewRay ray;
    ray.viewZenithCosine = clamp(cos(zenithAngle), -1.0, 1.0);
    ray.sunAzimuthCosine = clamp(1.0 - 2.0 * unitPosition.x * unitPosition.x, -1.0, 1.0);
    return ray;
}

// 視線から単位位置を求める。`rayFromSkyViewUnit`の逆写像である。
// 地平線のどちら側かは視線天頂余弦と地平線の余弦の比較だけで決まるため、呼び出し元は交差判定を渡さない。
float2 skyViewUnitFromRay(AtmosphereMedium medium, float observerRadius, SkyViewRay ray)
{
    float2 layout = skyViewVerticalLayout(medium, observerRadius);
    float zenithAngle = acos(clamp(ray.viewZenithCosine, -1.0, 1.0));
    float vertical;
    if (zenithAngle <= layout.x)
    {
        float ratio = skyViewSafeDivide(zenithAngle, layout.x);
        vertical = 0.5 * (1.0 - sqrt(max(0.0, 1.0 - ratio)));
    }
    else
    {
        float ratio = skyViewSafeDivide(zenithAngle - layout.x, layout.y);
        vertical = 0.5 + 0.5 * sqrt(max(0.0, ratio));
    }
    float horizontal = sqrt(max(0.0, (1.0 - ray.sunAzimuthCosine) * 0.5));
    return float2(clamp(horizontal, 0.0, 1.0), clamp(vertical, 0.0, 1.0));
}

// 書き込み先のテクセルの添字を、両端のテクセルが0と1に対応する単位位置へ写す。焼く側が使う。
float2 endToEndUnitPosition(uint2 texel, uint2 resolution)
{
    return float2(texel) / (float2(resolution) - 1.0);
}

// 単位位置を、その位置に対応するテクセル中心のUVへ写す。参照する側が使う。
float2 texelCenterUvFromUnit(float2 unitPosition, uint2 resolution)
{
    float2 size = float2(resolution);
    return (unitPosition * (size - 1.0) + 0.5) / size;
}
