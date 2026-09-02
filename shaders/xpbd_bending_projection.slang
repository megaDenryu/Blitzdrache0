// XPBDの面の曲げ拘束(隣接する2つの三角形の二面角)の正典式(判断11)のGPUの写し。CPUの参照計算
// `crates/blitz_sim/src/xpbd/bending_surface_projection.rs` と同じ符号・同じ単位・同じ演算の順で書く。
//   E = x_b − x_a、ê = E ÷ |E|、N1 = (x_a − x_c) × (x_b − x_c)、N2 = (x_b − x_d) × (x_a − x_d)、n = N ÷ |N|
//   θ = atan2(ê · (n2 × n1), n1 · n2)、C = (θ − 静止角)を常に最短の弧へ畳んだ角(ちょうど半周は正のπに決める)
//   g_c = |E| n1 ÷ |N1|、g_d = |E| n2 ÷ |N2|、g_a = ((x_c − x_b)·ê) n1 ÷ |N1| + ((x_d − x_b)·ê) n2 ÷ |N2|、g_b = −(((x_c − x_a)·ê) n1 ÷ |N1| + ((x_d − x_a)·ê) n2 ÷ |N2|)
//   W = Σ w_i |g_i|²、Δλ = (−C − α̃λ) ÷ (W + α̃)、補正_i = w_i Δλ g_i
// 退化(全て固定・辺が潰れている・三角形が潰れている)は補正0でΔλも0にし、呼び出し側が乗数を変えない。
// 弧の畳みは条件分岐で挟まずに常に通し、CPUの`ラジアン::一周の中で最短の弧へ畳む`(rem_euclid)と同じ演算の順(余りを取り、負なら一周を足し、半周を越えれば一周を引く)で書く。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 共有する辺の長さ(メートル)の下限。CPU正本の`向きが定まる最小の距離のメートル`(距離拘束と同じ)の写しであり、`cargo xtask conform`が突き合わせる。
// 長さの2乗でなく長さで比べるのは、CPU正本と同じ演算の順にし、定数の綴りをそのまま突き合わせられるようにするためである。
static const float minimumEdgeLength = 1e-6;
// 三角形の面積ベクトルの長さ(面積の2倍。平方メートル)の下限。CPU正本の`面が定まる最小の面積の平方メートル`の写しであり、`cargo xtask conform`が突き合わせる。
static const float minimumAreaVectorLength = 1e-12;
static const float pi = 3.14159265358979;

struct BendingProjection
{
    float3 correctionA;
    float3 correctionB;
    float3 correctionC;
    float3 correctionD;
    float deltaLambda;
    bool corrected;
};

// 角を−πからπまでの最短の弧へ畳む。CPUの`ラジアン::一周の中で最短の弧へ畳む`(rem_euclid)と同じ演算の順であり、余り(fmod。符号は被除数に従う)が負なら一周を足す。
// ちょうど半周は正のπに決める(−π → 余り−π → 一周を足してπ → πは半周を越えない → π)。
float wrapToShortestArc(float angle)
{
    float tau = 2.0 * pi;
    float remainder = fmod(angle, tau);
    float wrapped = remainder < 0.0 ? remainder + tau : remainder;
    return wrapped > pi ? wrapped - tau : wrapped;
}

BendingProjection projectDihedralBending(
    float3 positionA, float invMassA, float3 positionB, float invMassB, float3 positionC, float invMassC, float3 positionD, float invMassD,
    float restAngle, float alphaTilde, float lambda)
{
    BendingProjection result;
    result.correctionA = float3(0.0, 0.0, 0.0);
    result.correctionB = float3(0.0, 0.0, 0.0);
    result.correctionC = float3(0.0, 0.0, 0.0);
    result.correctionD = float3(0.0, 0.0, 0.0);
    result.deltaLambda = 0.0;
    result.corrected = false;
    if (invMassA + invMassB + invMassC + invMassD == 0.0)
    {
        return result;
    }
    float3 edge = positionB - positionA;
    float edgeLength = length(edge);
    if (edgeLength < minimumEdgeLength)
    {
        return result;
    }
    float3 area1 = cross(positionA - positionC, positionB - positionC);
    float3 area2 = cross(positionB - positionD, positionA - positionD);
    float area1Length = length(area1);
    float area2Length = length(area2);
    if (area1Length < minimumAreaVectorLength || area2Length < minimumAreaVectorLength)
    {
        return result;
    }
    float3 edgeDirection = edge / edgeLength;
    float3 normal1 = area1 / area1Length;
    float3 normal2 = area2 / area2Length;
    float angle = atan2(dot(edgeDirection, cross(normal2, normal1)), dot(normal1, normal2));
    float3 gradientC = normal1 * (edgeLength / area1Length);
    float3 gradientD = normal2 * (edgeLength / area2Length);
    float3 gradientA = normal1 * (dot(positionC - positionB, edgeDirection) / area1Length)
        + normal2 * (dot(positionD - positionB, edgeDirection) / area2Length);
    float3 gradientB = -(normal1 * (dot(positionC - positionA, edgeDirection) / area1Length)
        + normal2 * (dot(positionD - positionA, edgeDirection) / area2Length));
    float effectiveInverseInertia = invMassA * dot(gradientA, gradientA) + invMassB * dot(gradientB, gradientB)
        + invMassC * dot(gradientC, gradientC) + invMassD * dot(gradientD, gradientD);
    if (effectiveInverseInertia == 0.0)
    {
        return result;
    }
    float violation = wrapToShortestArc(angle - restAngle);
    float deltaLambda = (-violation - alphaTilde * lambda) / (effectiveInverseInertia + alphaTilde);
    result.deltaLambda = deltaLambda;
    result.correctionA = gradientA * (deltaLambda * invMassA);
    result.correctionB = gradientB * (deltaLambda * invMassB);
    result.correctionC = gradientC * (deltaLambda * invMassC);
    result.correctionD = gradientD * (deltaLambda * invMassD);
    result.corrected = true;
    return result;
}
