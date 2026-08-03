// 拡散照度の立方体画像の生成。テクセルが表す向きを法線として、遠方環境をLambert核で畳み込む。
// 書き込む量は照度の積分そのものであり、反射率を円周率で割った係数は標準PBRの側が掛ける。
// 余弦に比例した密度で標本するため、標本ごとの重みが円周率1つに畳まれる。
// 注意: 数式の正本は`crates/blitz_render/src/distant_environment/derived/diffuse_irradiance.rs`にあり、ここはその写しである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import cube_face_mapping;
import distant_environment_lookup;
import hammersley;
import sampling_math;
import tangent_basis;

[[vk::binding(0, 0)]] Texture2DArray<float4> distantEnvironment;
[[vk::binding(1, 0)]] [format("rgba16f")] RWTexture2DArray<float4> diffuseIrradiance;

// 1テクセルあたりの標本数。
// 注意: 正本の`拡散照度の標本数`と一致させる(`cargo xtask conform`が検査する)。
static const uint diffuseIrradianceSampleCount = 256;

// 半球の立体角。余弦分布の標本の重みがこの1つの定数へ畳まれる。
static const float diffuseIrradianceHemisphere = 3.14159265358979323846;

static const uint threadsPerSide = 8;

[shader("compute")]
[numthreads(threadsPerSide, threadsPerSide, 1)]
void computeMain(uint3 threadId : SV_DispatchThreadID)
{
    uint width;
    uint height;
    uint layers;
    diffuseIrradiance.GetDimensions(width, height, layers);
    if (threadId.x >= width || threadId.y >= height || threadId.z >= layers)
    {
        return;
    }

    float3 normal = directionOfCubeFace(threadId.z, cubeFaceTexelCenter(threadId.xy, width));
    TangentBasis basis = tangentBasisOf(normal);
    float3 total = float3(0.0, 0.0, 0.0);
    for (uint index = 0; index < diffuseIrradianceSampleCount; ++index)
    {
        float2 sample = hammersleySample(index, diffuseIrradianceSampleCount);
        float zenithCosine = sqrt(max(0.0, 1.0 - sample.x));
        float3 direction = tangentBasisToWorld(basis, normal, tangentSpaceDirection(zenithCosine, sample.y));
        total += distantEnvironmentTexel(distantEnvironment, direction);
    }

    float scale = diffuseIrradianceHemisphere / float(diffuseIrradianceSampleCount);
    diffuseIrradiance[threadId] = float4(total * scale, 0.0);
}
