// ハンマーズリー列の1標本。疑似乱数でなくこの列を使うのは、同じ条件で焼けば必ず同じ画素が出ることを
// 保証するためである。焼き直すかどうかを鍵の比較で決める設計は「同じ鍵なら同じ結果」に依存する。
// 注意: この列は`crates/blitz_render/src/distant_environment/derived/hammersley.rs`の写しである。正本はRust側にある。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 2の32乗の逆数。ビットを逆順に読み替えた整数へ掛けると単位区間の中の値になる。
static const float inverseTwoPow32 = 2.3283064365386963e-10;

// 前提: 標本数は1以上である(呼び出し元は畳み込みごとの定数を渡す)。
float2 hammersleySample(uint index, uint sampleCount)
{
    return float2(float(index) / float(sampleCount), float(reversebits(index)) * inverseTwoPow32);
}
