// 添字の重み付き総和を32ビット2語で持つ整数の器。CPU正本の`添字の重み付き総和`と同じ整数を作る。
// 注意: 64ビット整数を使わないのは、任意機能(shaderInt64)を新しい基礎要件にしないためである。
// 総和の上限は「窓の件数×最後のビンの添字」であり、32ビット1語では約3380万画素の画面で飽和する。
// 注意: 数式の正本は`crates/blitz_engine/src/auto_exposure/index_weight_sum.rs`にあり、ここはその写しである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import unfused_arithmetic;

// 上位の語が表す重み。2の32乗であり、単精度で厳密に表せる。
static const float highWordScale = 4294967296.0;

struct WeightedIndexSum
{
    uint low;
    uint high;
};

WeightedIndexSum zeroWeightedIndexSum()
{
    WeightedIndexSum sum;
    sum.low = 0;
    sum.high = 0;
    return sum;
}

// 1語を足し込む。下位が巻き戻ったら上位へ1つ繰り上げる。
WeightedIndexSum addWord(WeightedIndexSum sum, uint value)
{
    uint before = sum.low;
    sum.low = before + value;
    if (sum.low < before)
    {
        sum.high = sum.high + 1;
    }
    return sum;
}

// 1つのビンぶんを足す。重なりと添字の積は32ビットに収まらないため、重なりを16ビットずつに分けて足す。
// 分けた積はどちらも65535×127以下であり、32ビットに収まる。
WeightedIndexSum addTerm(WeightedIndexSum sum, uint overlap, uint index)
{
    uint lowerProduct = (overlap & 0xffff) * index;
    uint upperProduct = (overlap >> 16) * index;
    sum = addWord(sum, lowerProduct);
    sum = addWord(sum, upperProduct << 16);
    sum.high = sum.high + (upperProduct >> 16);
    return sum;
}

// 平均を作るための実数。上位の語への2の32乗の乗算は厳密であり、上位の語の変換・下位の語の変換・最後の加算が
// CPU正本と同じ順に起こる。
// 注意: 変換そのものが丸めを持つ。2の24乗を超える語は整数のまま単精度へ収まらないため、語の変換の時点で丸められる。
float weightedIndexSumAsFloat(WeightedIndexSum sum)
{
    return unfusedAdd(unfusedMultiply(float(sum.high), highWordScale), float(sum.low));
}
