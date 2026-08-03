// 反射率積分表の生成。分割和の第2項を、横が法線と視線の余弦、縦が粗さの2次元の表へ焼く。
// 第1成分は垂直入射の反射率に掛かる倍率、第2成分は反射率に依らず足す量である。
// 表は大気にも時刻にも依らないため、レンダラーの生成時に1度だけ焼く。
// 注意: 数式の正本は`crates/blitz_render/src/distant_environment/derived/brdf_integration.rs`と
// `brdf_axis.rs`にあり、ここはその写しである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import ggx_sample;
import hammersley;

[[vk::binding(0, 0)]] [format("rg16f")] RWTexture2D<float2> brdfIntegration;

// 1テクセルあたりの標本数。
// 注意: 正本の`反射率積分表の標本数`と一致させる(`cargo xtask conform`が検査する)。
static const uint brdfIntegrationSampleCount = 256;

static const uint threadsPerSide = 8;

// テクセルの添字を軸の値へ写す。テクセル中心を軸の値に取るのは、参照する側が余弦と粗さをそのままUVとして
// 標本するためである。
float brdfAxisTexelCenter(uint index, uint side)
{
    return (float(index) + 0.5) / float(side);
}

// Smithの遮蔽項の片側。係数は粗さの二乗の半分であり、遠方環境の畳み込み向けの取り方である。
float brdfSingleSideMasking(float cosine, float factor)
{
    float denominator = cosine * (1.0 - factor) + factor;
    return denominator <= 0.0 ? 0.0 : cosine / denominator;
}

// 遮蔽項を標本の密度で割った可視率。
float brdfVisibility(float viewCosine, float lightCosine, float halfCosine, float viewHalfCosine, float roughness)
{
    float factor = ggxLobeWidth(roughness) * 0.5;
    float masking = brdfSingleSideMasking(viewCosine, factor) * brdfSingleSideMasking(lightCosine, factor);
    float denominator = halfCosine * viewCosine;
    return denominator <= 0.0 ? 0.0 : masking * viewHalfCosine / denominator;
}

// 反射率が斜めの入射で1へ近づく分。5乗を掛け算の繰り返しで求めるのは、正本のRust側が同じ形(2乗の2乗に1つ掛ける)で
// 求めており、指数関数と対数関数を経由する`pow`が別の丸めを持つためである。この値は1から引かれるため、
// 視線が面に近い側では桁落ちして丸めの違いが数百倍に開く。
float reflectanceDropOf(float viewHalfCosine)
{
    float base = 1.0 - viewHalfCosine;
    float square = base * base;
    return square * square * base;
}

[shader("compute")]
[numthreads(threadsPerSide, threadsPerSide, 1)]
void computeMain(uint3 threadId : SV_DispatchThreadID)
{
    uint width;
    uint height;
    brdfIntegration.GetDimensions(width, height);
    if (threadId.x >= width || threadId.y >= height)
    {
        return;
    }

    float viewCosine = brdfAxisTexelCenter(threadId.x, width);
    float roughness = brdfAxisTexelCenter(threadId.y, height);
    float3 normal = float3(0.0, 0.0, 1.0);
    float3 view = float3(sqrt(max(0.0, 1.0 - viewCosine * viewCosine)), 0.0, viewCosine);

    float scaleSum = 0.0;
    float biasSum = 0.0;
    for (uint index = 0; index < brdfIntegrationSampleCount; ++index)
    {
        float3 halfVector = ggxHalfVectorSample(hammersleySample(index, brdfIntegrationSampleCount), roughness, normal);
        float3 lightDirection = reflectAroundHalfVector(view, halfVector);
        if (lightDirection.z <= 0.0)
        {
            continue;
        }
        float viewHalfCosine = max(0.0, view.x * halfVector.x + view.z * halfVector.z);
        float visibility = brdfVisibility(viewCosine, lightDirection.z, max(0.0, halfVector.z), viewHalfCosine, roughness);
        float reflectanceDrop = reflectanceDropOf(viewHalfCosine);
        scaleSum += (1.0 - reflectanceDrop) * visibility;
        biasSum += reflectanceDrop * visibility;
    }

    float inverse = 1.0 / float(brdfIntegrationSampleCount);
    brdfIntegration[threadId.xy] = float2(scaleSum * inverse, biasSum * inverse);
}
