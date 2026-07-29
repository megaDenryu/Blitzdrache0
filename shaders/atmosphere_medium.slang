// 大気媒体のGPU境界表現と、高度から規格化密度・視線に沿った光学距離・大気上端までの透過率を求める式。
// 注意: この式は`crates/blitz_render/src/atmosphere`の写しである。正本はRust側(density.rs・optical_length.rs・
// transmittance.rs・intersect.rs)にあり、閉形式との一致・正規化・単調性のユニットテストもRust側が持つ。
// ここが正本から外れると、LUTの色が静かにずれる。一致はGPU読み戻し検査が代表テクセルで確かめる。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 3成分(空気分子・浮遊粒子・吸収層)がそれぞれ2層の密度分布を持つ。
static const uint atmosphereComponentCount = 3;
static const uint atmosphereLayerCount = 6;
// 透過率LUTを焼くときに経路を刻む区間の数。Rust側の`transmittance_lut.rs`の標本区間数と同じ値でなければならない。
static const uint transmittanceStepCount = 40;

struct AtmosphereMedium
{
    // x=惑星下端半径, y=大気上端半径
    float4 radii;
    // 成分ごとの消散係数RGB。並びは空気分子・浮遊粒子・吸収層である。
    float4 extinction[atmosphereComponentCount];
    // 空気分子と浮遊粒子の散乱係数RGB。吸収層は散乱しないため持たない。
    float4 scattering[2];
    // xyz=地表アルベド, w=ミー位相の非対称係数
    float4 groundAlbedoAndAsymmetry;
    // 層ごとの x=幅, y=指数項, z=指数尺度, w=線形項
    float4 densityLayers[atmosphereLayerCount];
    // 層ごとの x=定数項
    float4 densityConstants[atmosphereLayerCount];
};

float bottomRadiusOf(AtmosphereMedium medium) { return medium.radii.x; }
float topRadiusOf(AtmosphereMedium medium) { return medium.radii.y; }

// 高度hの規格化密度を`指数項 * exp(指数尺度 * h) + 線形項 * h + 定数項`として与え、0以上1以下へ切り詰める。
// 指数項が0の層で指数を評価すると高度が大きいとき0と無限大の積になるため、項ごと落とす(Rust側の`density.rs`と同じ扱い)。
float evaluateLayer(float4 layer, float constantTerm, float altitude)
{
    float exponential = layer.y == 0.0 ? 0.0 : layer.y * exp(layer.z * altitude);
    return clamp(exponential + layer.w * altitude + constantTerm, 0.0, 1.0);
}

float componentDensity(AtmosphereMedium medium, uint component, float altitude)
{
    uint firstIndex = component * 2;
    uint index = altitude < medium.densityLayers[firstIndex].x ? firstIndex : firstIndex + 1;
    return evaluateLayer(medium.densityLayers[index], medium.densityConstants[index].x, altitude);
}

// 観測点から距離dだけ進んだ点の惑星中心からの距離の2乗は`d^2 + 2*r*mu*d + r^2`である。
// これを上端半径の2乗と等しいと置いた2次方程式の正の解が上端までの距離になる。
float distanceToTop(AtmosphereMedium medium, float radius, float zenithCosine)
{
    float top = topRadiusOf(medium);
    float discriminant = radius * radius * (zenithCosine * zenithCosine - 1.0) + top * top;
    return max(0.0, -radius * zenithCosine + sqrt(max(0.0, discriminant)));
}

// 台形則で規格化密度を線積分する。区間数nに対して標本点をn+1個取り、両端の重みを半分にする。
float opticalLengthToTop(AtmosphereMedium medium, uint component, float radius, float zenithCosine)
{
    float bottom = bottomRadiusOf(medium);
    float step = distanceToTop(medium, radius, zenithCosine) / float(transmittanceStepCount);
    float total = 0.0;
    for (uint index = 0; index <= transmittanceStepCount; ++index)
    {
        float traveled = float(index) * step;
        float sampleRadius = sqrt(max(0.0, traveled * traveled + 2.0 * radius * zenithCosine * traveled + radius * radius));
        float weight = (index == 0 || index == transmittanceStepCount) ? 0.5 : 1.0;
        total += componentDensity(medium, component, sampleRadius - bottom) * weight * step;
    }
    return total;
}

// 光学的深さは成分ごとの「消散係数 x 光学距離」の総和であり、透過率はBeer-Lambertの法則によりその負の指数である。
float3 transmittanceToTop(AtmosphereMedium medium, float radius, float zenithCosine)
{
    float3 opticalDepth = float3(0.0, 0.0, 0.0);
    for (uint component = 0; component < atmosphereComponentCount; ++component)
    {
        float length = opticalLengthToTop(medium, component, radius, zenithCosine);
        opticalDepth += medium.extinction[component].xyz * length;
    }
    return exp(-opticalDepth);
}
