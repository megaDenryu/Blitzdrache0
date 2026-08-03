// 鏡面畳込みの立方体画像の1つの粗さ段の生成。テクセルが表す向きを法線・視線・反射方向のすべてに取り、
// 遠方環境をGGXの分布で畳み込む。最詳細段は粗さ0であり、計算でなく遠方環境の複製で作るため、
// このシェーダーが走るのは第2段から先だけである。
// 注意: 数式の正本は`crates/blitz_render/src/distant_environment/derived/specular_prefilter.rs`にあり、ここはその写しである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import cube_face_mapping;
import distant_environment_lookup;
import ggx_sample;
import hammersley;

[[vk::binding(0, 0)]] Texture2DArray<float4> distantEnvironment;
[[vk::binding(1, 0)]] [format("rgba16f")] RWTexture2DArray<float4> specularPrefilter;

// この計算の発行が焼く段の粗さ。段と粗さの対応の正本は`鏡面畳込みの解像度`が持ち、シェーダーは結果だけを受け取る。
struct SpecularPrefilterCondition
{
    float roughness;
};
[[vk::push_constant]] SpecularPrefilterCondition specularPrefilterCondition;

// 1テクセルあたりの標本数。
// 注意: 正本の`鏡面畳込みの標本数`と一致させる(`cargo xtask conform`が検査する)。
static const uint specularPrefilterSampleCount = 128;

static const uint threadsPerSide = 8;

[shader("compute")]
[numthreads(threadsPerSide, threadsPerSide, 1)]
void computeMain(uint3 threadId : SV_DispatchThreadID)
{
    uint width;
    uint height;
    uint layers;
    specularPrefilter.GetDimensions(width, height, layers);
    if (threadId.x >= width || threadId.y >= height || threadId.z >= layers)
    {
        return;
    }

    float3 normal = directionOfCubeFace(threadId.z, cubeFaceTexelCenter(threadId.xy, width));
    float3 total = float3(0.0, 0.0, 0.0);
    float weightSum = 0.0;
    for (uint index = 0; index < specularPrefilterSampleCount; ++index)
    {
        float2 sample = hammersleySample(index, specularPrefilterSampleCount);
        float3 halfVector = ggxHalfVectorSample(sample, specularPrefilterCondition.roughness, normal);
        float3 lightDirection = reflectAroundHalfVector(normal, halfVector);
        float cosine = dot(normal, lightDirection);
        if (cosine <= 0.0)
        {
            continue;
        }
        total += distantEnvironmentTexel(distantEnvironment, lightDirection) * cosine;
        weightSum += cosine;
    }

    // 重みが1つも立たなかったときに0を書くと画素が黒く落ちるため、畳み込まない元の値を書く。
    float3 filtered = weightSum > 0.0 ? total / weightSum : distantEnvironmentTexel(distantEnvironment, normal);
    specularPrefilter[threadId] = float4(filtered, 0.0);
}
