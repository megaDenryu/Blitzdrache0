// M9: 布シミュレーションの介入適用と積分(判断52〜54)。
// 布シミュ全シェーダー共通のバインディング表(統一ディスクリプタセット。各ファイルは使う分だけ宣言する):
//   b0=シミュ定数UBO(進行中フレーム2重) b1=粒子(vec4x2/粒子: pos.w=逆質量, vel.w=予備)
//   b2=前位置(vec4/粒子) b3=介入キュー(32B/件、2重) b4=拘束の引数(uint2+float2/拘束: a添字・b添字・静止長・コンプライアンス。彩色で色ごとに並べ替えた順)
//   b5=セルカウント(u32/セル) b6=セル格納(u32x8/セル) b7=布頂点(float12個/頂点=48B)
//   b8=スキン済み頂点 b9=アタッチ対応 b10=ラグランジュ乗数(float/拘束。刻みの開始で0へ戻す)
// 束縛番号の正本は`crates/blitz_render/src/vulkan/cloth/descriptor.rs`、拘束と乗数のレイアウトの正本は`crates/blitz_sim/src/gpu_layout/xpbd.rs`である。
// 介入レイアウト(blitz_sim::interventionの仕様と一致): 種別u32(0=掴む,1=離す,2=インパルス)+
// 粒子添字u32+パディングu32x2+float4(掴む=目標位置xyz / インパルス=xyz力)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct ClothParams
{
    float4 gravityDt;    // xyz=重力加速度, w=dt
    float4 gridOrigin;   // xyz=空間グリッド原点, w=セル寸法
    float4 capsuleA;     // xyz=カプセル端点A, w=カプセル半径
    float4 capsuleB;     // xyz=カプセル端点B, w=既定逆質量(離す介入の復元値)
    uint4 counts;        // x=粒子数, y=一辺粒子数, z=介入件数, w=アタッチ対応の件数
    float4 misc;         // x=粒子衝突半径, y=1÷刻み幅²(コンプライアンスから刻み依存量α̃を導く), z=速度減衰, w=予備
};

[[vk::binding(0, 0)]] ConstantBuffer<ClothParams> params;
[[vk::binding(1, 0)]] RWStructuredBuffer<float4> particles;
[[vk::binding(2, 0)]] RWStructuredBuffer<float4> previousPositions;
struct Intervention
{
    uint kind;
    uint particleIndex;
    uint2 padding;
    float4 value;
};
[[vk::binding(3, 0)]] StructuredBuffer<Intervention> interventions;

// 掴む(種別0)=位置と前位置を目標へ移して固定(逆質量0)。離す(種別1)=逆質量を既定値へ復元。
// インパルス(種別2)はM9では未使用のため予約(何もしない)。32バイトの現レイアウトには位置と力が
// 同時に入らないため、M10でレイアウト拡張と合わせて実装する(無言の別解釈で動かすことを避ける)。
[shader("compute")]
[numthreads(64, 1, 1)]
void interventionMain(uint3 id: SV_DispatchThreadID)
{
    uint index = id.x;
    if (index >= params.counts.z)
    {
        return;
    }
    Intervention item = interventions[index];
    if (item.kind == 0)
    {
        particles[item.particleIndex * 2] = float4(item.value.xyz, 0.0);
        particles[item.particleIndex * 2 + 1] = float4(0.0, 0.0, 0.0, 0.0);
        previousPositions[item.particleIndex] = float4(item.value.xyz, 0.0);
    }
    else if (item.kind == 1)
    {
        particles[item.particleIndex * 2].w = params.capsuleB.w;
    }
}

// 半陰的オイラー: 速度に重力を積み、前位置を保存して位置を進める。固定粒子(逆質量0)は動かさない。
[shader("compute")]
[numthreads(64, 1, 1)]
void integrateMain(uint3 id: SV_DispatchThreadID)
{
    uint index = id.x;
    if (index >= params.counts.x)
    {
        return;
    }
    float4 positionInvMass = particles[index * 2];
    previousPositions[index] = float4(positionInvMass.xyz, 0.0);
    if (positionInvMass.w == 0.0)
    {
        return;
    }
    float3 velocity = particles[index * 2 + 1].xyz + params.gravityDt.xyz * params.gravityDt.w;
    particles[index * 2 + 1].xyz = velocity;
    particles[index * 2].xyz = positionInvMass.xyz + velocity * params.gravityDt.w;
}
