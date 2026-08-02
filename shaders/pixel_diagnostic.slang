// シーンの画素段が本番の色の代わりに出す診断出力。担当するのは、診断の種別の判別と、種別ごとの1画素の色である。
// 影の評価そのものは呼び出し側が済ませ、ここは受け取った影可視度を符号化するだけである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import cascade_shadow;

// 診断の種別。CascadeShadowUniform.cascadeDebugAndTexelSize.xが運ぶ値である。
// 注意: 値の正本は`crates/blitz_render/src/cascade/diagnostic.rs`の`画素診断::番号`であり、`cargo xtask conform`が一致を見る。
static const float pixelDiagnosticCascadeBands = 1.0;
static const float pixelDiagnosticShadowLoss = 2.0;

// 影の欠落計器が使う受光距離帯の刻みと帯の数。
// 注意: 値の正本は`crates/blitz_render/src/cascade/diagnostic.rs`であり、`cargo xtask conform`が一致を見る。
static const float receivingBandWidthMeters = 25.0;
static const uint receivingBandCount = 16;

// 距離区分の可視化色。距離区分ごとに使うチャンネルの組を変え、影の中では同じ色を暗い段へ落とす。
// 検収はチャンネルの組で距離区分を、明るさの段で影の内外を読み取る。2段しか作らないのは、
// 中間の値を作ると読み取り側が距離区分と影を分けられなくなるためである。
float3 cascadeDebugColor(uint cascadeIndex, float shadowFactor)
{
    float3 bandColor = float3(0.0, 0.0, 0.0);
    if (cascadeIndex == 0) bandColor = float3(1.0, 0.0, 0.0);
    else if (cascadeIndex == 1) bandColor = float3(0.0, 1.0, 0.0);
    else if (cascadeIndex == 2) bandColor = float3(0.0, 0.0, 1.0);
    else bandColor = float3(1.0, 1.0, 0.0);
    return bandColor * (shadowFactor > 0.5 ? 1.0 : 0.25);
}

// 影の欠落計器の1画素。赤に影可視度をそのまま載せ、緑に受光距離帯を、青に幾何の印を載せる。
// 受光距離帯を多段の分割でなく固定の刻みで決めるのは、最大影距離を振ると分割深度が動き、区分を帯に使うと
// 基準と候補で帯の意味が変わって「どこの影が失われたか」を比べられなくなるためである。
// 帯は0から始まる整数であり、最後の帯は上限を持たない。読み取り側が丸めで戻せるよう(帯+1)/帯数として載せる。
// 注意: 出力は交換チェーンのsRGB符号化を通るため、読み取り側は同じ伝達関数の逆で線形へ戻す。
float3 shadowLossDiagnosticColor(float receivingDistance, float shadowFactor)
{
    float band = min(floor(receivingDistance / receivingBandWidthMeters), float(receivingBandCount - 1));
    return float3(shadowFactor, (band + 1.0) / float(receivingBandCount), 1.0);
}

// 診断の種別が選んだ1画素の色。種別が「出さない」なら偽を返し、呼び出し側は本番の色を出す。
bool pixelDiagnosticColor(
    float diagnosticKind, float viewDepth, float3 splitDepths, float3 transitionWidths, float shadowFactor,
    float receivingDistance, out float3 diagnosticColor)
{
    if (diagnosticKind > pixelDiagnosticShadowLoss - 0.5)
    {
        diagnosticColor = shadowLossDiagnosticColor(receivingDistance, shadowFactor);
        return true;
    }
    if (diagnosticKind > pixelDiagnosticCascadeBands - 0.5)
    {
        uint cascadeIndex = selectCascadeBlend(viewDepth, splitDepths, transitionWidths).primaryIndex;
        diagnosticColor = cascadeDebugColor(cascadeIndex, shadowFactor);
        return true;
    }
    diagnosticColor = float3(0.0, 0.0, 0.0);
    return false;
}
