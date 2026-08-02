// GPU粒子トイ(M5波2判断29)。ストレージバッファをコンピュートで更新し、
// 頂点シェーダーが同じバッファをSV_VertexIDで読んで点として描画する
// (レンダーグラフのコンピュート->グラフィックス間バッファ共有の実証)。
// 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`判断29、`_doc/設計/レンダーグラフ.md`。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import relative_anchor;
import view_pass_uniform;

struct Particle
{
    float4 position;
    float4 velocity;
};

// binding3: コンピュートは読み書き(RWStructuredBuffer)、頂点は読み取りのみ
// (StructuredBuffer)。同じdescriptor type(storage buffer)なので同一bindingを
// 指せる。頂点側をRWStructuredBufferにすると`vertexPipelineStoresAndAtomics`
// 機能が無効な環境でNonWritable違反のvalidationエラーになるため型を分ける。
[[vk::binding(3, 0)]]
RWStructuredBuffer<Particle> particlesRW;

[[vk::binding(3, 0)]]
StructuredBuffer<Particle> particlesRO;

static const float3 GRAVITY = float3(0.0, -1.5, 0.0);
static const float FLOOR_Y = -1.0;
static const float BOUNCE = 0.5;
static const float DELTA_TIME = 1.0 / 60.0;

[shader("compute")]
[numthreads(64, 1, 1)]
void computeMain(uint3 id : SV_DispatchThreadID)
{
    Particle particle = particlesRW[id.x];
    particle.velocity.xyz += GRAVITY * DELTA_TIME;
    particle.position.xyz += particle.velocity.xyz * DELTA_TIME;
    if (particle.position.y < FLOOR_Y)
    {
        particle.position.y = FLOOR_Y;
        particle.velocity.y *= -BOUNCE;
    }
    particlesRW[id.x] = particle;
}

struct ParticleVertexOutput
{
    float4 position : SV_Position;
    float pointSize : SV_PointSize;
};

[shader("vertex")]
ParticleVertexOutput vertexMain(uint vertexID : SV_VertexID)
{
    ParticleVertexOutput output;
    float3 particlePosition = applyRelativeAnchor(particlesRO[vertexID].position.xyz);
    output.position = mul(viewPassUniform.viewProjection, float4(particlePosition, 1.0));
    output.pointSize = 4.0;
    return output;
}

[shader("fragment")]
float4 fragmentMain() : SV_Target
{
    return float4(1.0, 0.9, 0.2, 1.0);
}
