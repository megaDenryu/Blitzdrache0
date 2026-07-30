// 多重散乱のベイク済み画像のテクセル座標と、太陽の向き・観測点の半径との行き来。焼く側と参照する側の両方がここを読む。
// 注意: この式は`crates/blitz_render/src/atmosphere/mapping/multiscatter_mapping.rs`の写しである。正本はRust側にあり、
// 写像と逆写像の往復のユニットテストもそちらが持つ。ここが正本から外れると、焼いた座標と参照する座標が食い違って
// 多重散乱の項だけが静かにずれる。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_medium;

// 最下行を地表から離す割合と、最上行を大気上端から離す量。端点の経路長が0になるのを避ける。
static const float multiScatterLowestRowLift = 0.01;
static const float multiScatterHighestRowDropMeters = 10.0;

struct MultiScatterCondition
{
    float sunZenithCosine;
    float radius;
};

// 単位位置(両端のテクセルが0と1に対応する位置)から、そのテクセルが表す太陽の向きと観測点の半径を求める。
MultiScatterCondition multiScatterConditionFromUnit(AtmosphereMedium medium, float2 unitPosition)
{
    float bottom = bottomRadiusOf(medium);
    float top = topRadiusOf(medium);
    float heightRatio = min(unitPosition.y + multiScatterLowestRowLift, 1.0);

    MultiScatterCondition condition;
    condition.sunZenithCosine = clamp(unitPosition.x * 2.0 - 1.0, -1.0, 1.0);
    condition.radius = clamp(bottom + heightRatio * (top - bottom - multiScatterHighestRowDropMeters), bottom, top);
    return condition;
}

// 太陽の向きと観測点の半径から単位位置を求める。`multiScatterConditionFromUnit`の逆写像である。
// 最上行は焼く側が高さの割合を1で頭打ちにするためそこだけ単射でなく、この逆写像は0.99へ写る。
float2 multiScatterUnitFromCondition(AtmosphereMedium medium, float sunZenithCosine, float radius)
{
    float bottom = bottomRadiusOf(medium);
    float top = topRadiusOf(medium);
    float usableThickness = top - bottom - multiScatterHighestRowDropMeters;
    float horizontal = clamp(sunZenithCosine * 0.5 + 0.5, 0.0, 1.0);
    float vertical = usableThickness <= 0.0
        ? 0.0
        : clamp((radius - bottom) / usableThickness - multiScatterLowestRowLift, 0.0, 1.0);
    return float2(horizontal, vertical);
}
