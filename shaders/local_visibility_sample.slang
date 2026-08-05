// 標本1本ぶんの向きを求める工程。受け取るのは標本の添字と本数とその画素の回転と法線、返すのは
// 法線を軸とする半球の面の上のビュー空間の向きである。
// 平面の長さを第二成分の平方根に取ると、向きの分布は法線との余弦に比例する。標本を法線の近くへ厚く置く
// この分布は、余弦を掛けて積分する拡散間接の遮蔽の形と合っており、同じ標本数でむらが小さい。
// 注意: この式は`crates/blitz_render/src/local_visibility/hemisphere_sample.rs`の写しである。正本はRust側にある。
// 注意: ハンマーズリー列と接基底は派生表現の畳み込みが持つ正本を共有する。同じ数学へ2つの正本を置くと
// 値の一致検査が二重になるためである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import hammersley;
import local_visibility_rotation;
import tangent_basis;

// 半球の面の上の標本の向き。返す向きと法線の内積は必ず0以上である。
// 前提: 添字は標本数未満であり、法線は長さ1に正規化されている。
float3 localVisibilityHemisphereDirection(uint index, uint sampleCount, LocalVisibilityRotation rotation, float3 normal)
{
    float2 fraction = hammersleySample(index, sampleCount);
    float planeLength = sqrt(fraction.y);
    float normalLength = sqrt(1.0 - fraction.y);
    float azimuth = localVisibilityFullTurn * fraction.x;
    float2 rotated = localVisibilityRotatePlane(rotation, float2(cos(azimuth), sin(azimuth)));
    float3 tangentDirection = float3(planeLength * rotated.x, planeLength * rotated.y, normalLength);
    TangentBasis basis = tangentBasisOf(normal);
    return tangentBasisToWorld(basis, normal, tangentDirection);
}
