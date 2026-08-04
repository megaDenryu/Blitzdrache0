// 自動露出のヒストグラム集計。光のにじみを加える前のHDR中間画像を1画素ずつ読み、ビンと2つの枠へ数え上げる。
// 注意: 集計の定義の正本は`crates/blitz_engine/src/auto_exposure/aggregation.rs`にあり、ここはその写しである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import auto_exposure_luminance;
import auto_exposure_scale;

[[vk::binding(0, 0)]] Texture2D<float4> hdrImage;
[[vk::binding(1, 0)]] StructuredBuffer<float> binBoundaries;
[[vk::binding(2, 0)]] RWStructuredBuffer<uint> histogram;

// 集計するHDR中間画像の寸法。画像から取ることもできるが、消去パスと同じ寸法を1つの値で配るために即時定数で渡す。
struct HistogramExtent
{
    uint width;
    uint height;
};
[[vk::push_constant]] HistogramExtent histogramExtent;

static const uint threadsPerSide = 16;
static const uint threadsPerGroup = threadsPerSide * threadsPerSide;

// 班ごとの数え上げ。全画素が直接1本のバッファへ原子的加算を積むと130個の位置へ競合が集中するため、
// 班の中で数えてから1回だけ足し込む。整数の数え上げであり、まとめる順で結果は変わらない。
groupshared uint groupHistogram[histogramSlotCount];

[shader("compute")]
[numthreads(threadsPerSide, threadsPerSide, 1)]
void computeMain(uint3 threadId : SV_DispatchThreadID, uint groupThreadIndex : SV_GroupIndex)
{
    for (uint slot = groupThreadIndex; slot < histogramSlotCount; slot += threadsPerGroup)
    {
        groupHistogram[slot] = 0;
    }
    GroupMemoryBarrierWithGroupSync();

    if (threadId.x < histogramExtent.width && threadId.y < histogramExtent.height)
    {
        float3 color = hdrImage.Load(int3(int(threadId.x), int(threadId.y), 0)).rgb;
        InterlockedAdd(groupHistogram[findHistogramSlot(binBoundaries, color)], 1);
    }
    GroupMemoryBarrierWithGroupSync();

    for (uint flushed = groupThreadIndex; flushed < histogramSlotCount; flushed += threadsPerGroup)
    {
        uint count = groupHistogram[flushed];
        if (count != 0)
        {
            InterlockedAdd(histogram[flushed], count);
        }
    }
}
