// 天頂の余弦と方位の割合から接空間の向きを組む。拡散照度の余弦分布とGGXの分布は天頂余弦の決め方だけが違い、
// そこから向きを組む段は同じである。2箇所へ書くと、一周の定数や正弦の求め方が片方だけ変わったときに
// 2つの畳み込みが別の座標系を使う。
// 注意: この式は`crates/blitz_render/src/distant_environment/derived/spherical_direction.rs`の写しである。正本はRust側にある。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 一周のラジアン。方位の割合へ掛けて方位角にする。
static const float samplingFullTurn = 6.28318530717958647692;

float3 tangentSpaceDirection(float zenithCosine, float azimuthFraction)
{
    float zenithSine = sqrt(max(0.0, 1.0 - zenithCosine * zenithCosine));
    float azimuth = samplingFullTurn * azimuthFraction;
    return float3(zenithSine * cos(azimuth), zenithSine * sin(azimuth), zenithCosine);
}
