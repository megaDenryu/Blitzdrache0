// GGXの法線分布から半ベクトルを1本標本する。一様に標本して分布で重み付けるのでなく分布そのものから
// 標本するのは、粗さが小さいほど分布が鋭くなり、一様標本ではほとんどの標本が0に近い重みしか持たなくなるためである。
// 鏡面の畳み込みと反射率積分表の両方がこの1つの標本を使う。
// 注意: この標本は`crates/blitz_render/src/distant_environment/derived/ggx_sample.rs`の写しである。正本はRust側にある。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import sampling_math;
import tangent_basis;

// 粗さから法線分布の広がりを求める。粗さの二乗を取るのは、粗さを見た目の変化が一様になる目盛りとして持つ
// Disneyの規約に合わせるためである。
float ggxLobeWidth(float roughness)
{
    return roughness * roughness;
}

// 分布の累積を逆に解いて天頂余弦を求める。広がりが0のとき分母と分子がともに1-標本になるため、
// 余弦は1(半ベクトルが法線と一致する完全な鏡面)になる。
float ggxZenithCosine(float sample, float lobeWidth)
{
    float denominator = 1.0 + (lobeWidth * lobeWidth - 1.0) * sample;
    if (denominator <= 0.0)
    {
        return 1.0;
    }
    return sqrt(clamp((1.0 - sample) / denominator, 0.0, 1.0));
}

// 標本1つから半ベクトルを求める。前提: 法線は長さ1に正規化されている。
float3 ggxHalfVectorSample(float2 sample, float roughness, float3 normal)
{
    float3 tangentDirection = tangentSpaceDirection(ggxZenithCosine(sample.x, ggxLobeWidth(roughness)), sample.y);
    return tangentBasisToWorld(tangentBasisOf(normal), normal, tangentDirection);
}

// 半ベクトルで視線を反射させて光の向きを求める。視線と法線が一致する畳み込みでも、
// 反射率積分表の斜めの視線でも同じ式である。
float3 reflectAroundHalfVector(float3 view, float3 halfVector)
{
    return 2.0 * dot(view, halfVector) * halfVector - view;
}
