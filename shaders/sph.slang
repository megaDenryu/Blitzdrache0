// M10の弱圧縮性SPH技術検証。1ワークグループが全粒子を担当し、全対全探索の計算量を測る。
// 前半を現在状態、後半を更新先として使い、グループ内デバイスバリア後に前半へ戻す。

import relative_anchor;

struct Particle
{
    float4 positionDensity;
    float4 velocityCount;
};

[[vk::binding(0, 0)]] RWStructuredBuffer<Particle> particlesRW;
[[vk::binding(0, 0)]] StructuredBuffer<Particle> particlesRO;

struct FrameUniform
{
    column_major float4x4 cascadeLightViewProjection[4];
    column_major float4x4 viewProjection;
};

[[vk::binding(1, 0)]] ConstantBuffer<FrameUniform> frameUniform;

static const float DELTA_TIME = 1.0 / 120.0;
static const float SMOOTHING_RADIUS = 0.24;
static const float REST_DENSITY = 0.08;
static const float PRESSURE_SCALE = 5.0;
static const float VISCOSITY = 2.0;

float densityWeight(float distance)
{
    float q = max(0.0, 1.0 - distance / SMOOTHING_RADIUS);
    return q * q * q;
}

[shader("compute")]
[numthreads(64, 1, 1)]
void computeMain(uint3 id : SV_DispatchThreadID)
{
    uint count = uint(particlesRW[0].velocityCount.w + 0.5);
    float mass = 512.0 / float(count);
    for (uint i = id.x; i < count; i += 64)
    {
        float density = 0.0;
        for (uint j = 0; j < count; ++j)
            density += densityWeight(length(particlesRW[i].positionDensity.xyz - particlesRW[j].positionDensity.xyz)) * mass;
        particlesRW[count + i].positionDensity.w = density;
    }
    DeviceMemoryBarrierWithGroupSync();
    for (uint i = id.x; i < count; i += 64)
    {
        Particle current = particlesRW[i];
        float density = particlesRW[count + i].positionDensity.w;
        float3 acceleration = float3(0.0, -2.0, 0.0);
        for (uint j = 0; j < count; ++j)
        {
            if (i == j) continue;
            float3 difference = current.positionDensity.xyz - particlesRW[j].positionDensity.xyz;
            float distance = length(difference);
            if (distance <= 0.0001 || distance >= SMOOTHING_RADIUS) continue;
            float weight = 1.0 - distance / SMOOTHING_RADIUS;
            float neighborDensity = particlesRW[count + j].positionDensity.w;
            float pressure = max(0.0, density + neighborDensity - 2.0 * REST_DENSITY) * PRESSURE_SCALE;
            acceleration += normalize(difference) * pressure * weight * mass / max(neighborDensity, 0.01);
            acceleration += (particlesRW[j].velocityCount.xyz - current.velocityCount.xyz) * VISCOSITY * weight * mass;
        }
        Particle output = current;
        output.velocityCount.xyz = clamp(current.velocityCount.xyz + acceleration * DELTA_TIME, -4.0, 4.0);
        output.positionDensity.xyz += output.velocityCount.xyz * DELTA_TIME;
        if (output.positionDensity.y < -1.0) { output.positionDensity.y = -1.0; output.velocityCount.y *= -0.3; }
        if (abs(output.positionDensity.x) > 1.5) { output.positionDensity.x = clamp(output.positionDensity.x, -1.5, 1.5); output.velocityCount.x *= -0.3; }
        if (abs(output.positionDensity.z) > 1.5) { output.positionDensity.z = clamp(output.positionDensity.z, -1.5, 1.5); output.velocityCount.z *= -0.3; }
        output.positionDensity.w = density;
        particlesRW[count + i] = output;
    }
    DeviceMemoryBarrierWithGroupSync();
    for (uint i = id.x; i < count; i += 64) particlesRW[i] = particlesRW[count + i];
}

struct VertexOutput { float4 position : SV_Position; float pointSize : SV_PointSize; };

[shader("vertex")]
VertexOutput vertexMain(uint vertexID : SV_VertexID)
{
    VertexOutput output;
    float3 relativePosition = applyRelativeAnchor(particlesRO[vertexID].positionDensity.xyz);
    output.position = mul(frameUniform.viewProjection, float4(relativePosition, 1.0));
    output.pointSize = 4.0;
    return output;
}

[shader("fragment")]
float4 fragmentMain() : SV_Target { return float4(0.1, 0.55, 1.0, 1.0); }
