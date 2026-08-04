// 積和融合(乗算と加算を1命令にまとめ、積の丸めを省く最適化)を許さない単精度の四則。
// CPU正本と同じ演算の順・同じ丸めの回数を生成物の水準で保証するために使う。
//
// 注意: `precise`修飾では足りない。slangc 2025.11時点の生成SPIR-Vには`precise`が1つも装飾を残さず
// (spirv-disで確認済み)、融合しない契約をドライバーへ渡せない。ここはSPIR-Vのインライン記述で
// OpFMul・OpFAdd・OpFSubへNoContractionを直に付ける。生成物に装飾が実在することは
// `crates/blitz_app/src/embedded_shaders/no_contraction_tests.rs`が機械的に見る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 融合しない乗算。
float unfusedMultiply(float left, float right)
{
    return spirv_asm {
        %product : $$float = OpFMul $left $right;
        OpDecorate %product NoContraction;
        result:$$float = OpCopyObject %product
    };
}

// 融合しない加算。
float unfusedAdd(float left, float right)
{
    return spirv_asm {
        %sum : $$float = OpFAdd $left $right;
        OpDecorate %sum NoContraction;
        result:$$float = OpCopyObject %sum
    };
}

// 融合しない減算。
float unfusedSubtract(float left, float right)
{
    return spirv_asm {
        %difference : $$float = OpFSub $left $right;
        OpDecorate %difference NoContraction;
        result:$$float = OpCopyObject %difference
    };
}
