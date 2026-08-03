// 単位ベクトル1本から正規直交の接基底2本を組む。分岐で上方向を選ぶ素朴な組み方を採らないのは、
// 法線が上方向と平行になる極で外積が0へ潰れ、その近くで基底が跳ねるためである。
// 使うのはDuffらの方法であり、法線の第三成分の符号だけで場合が分かれ、どの向きでも分母が1以上に保たれる。
// 注意: この組み方は`crates/blitz_render/src/distant_environment/derived/tangent_basis.rs`の写しである。正本はRust側にある。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct TangentBasis
{
    float3 firstAxis;
    float3 secondAxis;
};

// 前提: 法線は長さ1に正規化されている。
TangentBasis tangentBasisOf(float3 normal)
{
    float axisSign = normal.z >= 0.0 ? 1.0 : -1.0;
    float inverse = -1.0 / (axisSign + normal.z);
    float crossTerm = normal.x * normal.y * inverse;

    TangentBasis basis;
    basis.firstAxis = float3(1.0 + axisSign * normal.x * normal.x * inverse, axisSign * crossTerm, -axisSign * normal.x);
    basis.secondAxis = float3(crossTerm, axisSign + normal.y * normal.y * inverse, -normal.y);
    return basis;
}

// 接基底の座標で表した向きをもとの座標へ戻す。第一成分と第二成分が接基底、第三成分が法線に掛かる。
float3 tangentBasisToWorld(TangentBasis basis, float3 normal, float3 tangentDirection)
{
    return basis.firstAxis * tangentDirection.x + basis.secondAxis * tangentDirection.y + normal * tangentDirection.z;
}
