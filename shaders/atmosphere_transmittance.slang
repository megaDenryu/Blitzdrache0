// 透過率のベイク済み画像の生成。テクセルごとに観測点と視線を復元し、そこから大気上端までの透過率を焼く。
// 解像度の定数を持たず書き込み先の寸法を問い合わせるのは、Rust側の品質方針型と二重管理にしないためである。
// 注意: 数式の正本は`crates/blitz_render/src/atmosphere`にあり、ここはその写しである(詳細はimport先の冒頭)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import atmosphere_mapping;
import atmosphere_medium;

[[vk::binding(0, 0)]] ConstantBuffer<AtmosphereMedium> atmosphereMedium;
[[vk::binding(1, 0)]] [format("rgba16f")] RWTexture2D<float4> transmittanceLut;

static const uint threadsPerSide = 8;

[shader("compute")]
[numthreads(threadsPerSide, threadsPerSide, 1)]
void computeMain(uint3 threadId : SV_DispatchThreadID)
{
    uint width;
    uint height;
    transmittanceLut.GetDimensions(width, height);
    if (threadId.x >= width || threadId.y >= height)
    {
        return;
    }

    float2 uv = texelCenterUv(threadId.xy, uint2(width, height));
    AtmosphereRay ray = rayFromTransmittanceUv(atmosphereMedium, uv);
    float3 transmittance = transmittanceToTop(atmosphereMedium, ray.radius, ray.zenithCosine);
    // アルファは予約であり、後段が読むのはRGBだけである。読み戻し検査が有限性を見るため0を明示して書く。
    transmittanceLut[threadId.xy] = float4(transmittance, 0.0);
}
