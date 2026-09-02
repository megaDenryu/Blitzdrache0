// XPBDの距離拘束の正典式(判断1)のGPUの写し。CPUの参照計算
// `crates/blitz_sim/src/xpbd/distance_canonical_projection.rs` と同じ符号・同じ単位・同じ演算の順で書く。
//   d = b − a、n = d ÷ |d|、C = |d| − 静止長、Δλ = (−C − α̃λ) ÷ (wa + wb + α̃)、補正a = −wa Δλ n、補正b = +wb Δλ n
// 退化(両方の逆質量が0・距離が下限未満)は補正0でΔλも0にし、呼び出し側が乗数を変えない。
// 3つの並列方式(原子加算・グラフ彩色・二段階)がこの1つの関数を読むため、方式の違いは式の違いにならない。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 2点の距離の2乗の下限。CPU正本が長さに置く下限1e-6メートルの2乗であり、現在の布と同じ値である。
static const float minimumSquaredDistance = 1e-12;

struct DistanceProjection
{
    float3 correctionA;
    float3 correctionB;
    float deltaLambda;
    bool corrected;
};

DistanceProjection projectDistance(
    float3 positionA, float invMassA, float3 positionB, float invMassB, float restLength, float alphaTilde, float lambda)
{
    DistanceProjection result;
    result.correctionA = float3(0.0, 0.0, 0.0);
    result.correctionB = float3(0.0, 0.0, 0.0);
    result.deltaLambda = 0.0;
    result.corrected = false;
    float weightSum = invMassA + invMassB;
    if (weightSum == 0.0)
    {
        return result;
    }
    float3 difference = positionB - positionA;
    float squaredLength = dot(difference, difference);
    if (squaredLength < minimumSquaredDistance)
    {
        return result;
    }
    float currentLength = sqrt(squaredLength);
    float3 direction = difference / currentLength;
    float violation = currentLength - restLength;
    float deltaLambda = (-violation - alphaTilde * lambda) / (weightSum + alphaTilde);
    result.deltaLambda = deltaLambda;
    result.correctionA = direction * (-invMassA * deltaLambda);
    result.correctionB = direction * (invMassB * deltaLambda);
    result.corrected = true;
    return result;
}

// 全拘束を同じ位置から同時に射影する方式(原子加算・二段階)の緩和係数。両端の接続数の大きい方の逆数であり、
// CPUの参照計算 `crates/blitz_sim/src/constraint_graph/reference/schedule.rs` と同じ式である。
// 乗数の増分と補正の両方へ掛け、乗数が「実際に加えた補正の総量」であるという正典の意味を保つ。
float relaxationFactor(uint degreeA, uint degreeB)
{
    return 1.0 / float(max(max(degreeA, degreeB), 1u));
}

// 隣接表の1語(拘束添字の2倍に側を足した値)から接続数を引く区間の幅を求める。
uint degreeOf(StructuredBuffer<uint> adjacencyOffsets, uint pointIndex)
{
    return adjacencyOffsets[pointIndex + 1] - adjacencyOffsets[pointIndex];
}
