// M9: 布シミュレーションの介入適用と積分(判断52〜54)。
// 布シミュ全シェーダー共通のバインディング表(統一ディスクリプタセット。各ファイルは使う分だけ宣言する):
//   b0=シミュ定数UBO(進行中フレーム2重) b1=粒子(vec4x2/粒子: pos.w=逆質量, vel.w=予備)
//   b2=前位置(vec4/粒子) b3=介入キュー(32B/件、2重) b4=拘束の引数(uint2+float2/拘束: a添字・b添字・静止長・コンプライアンス。彩色で色ごとに並べ替えた順)
//   b5=セルカウント(u32/セル) b6=セル格納(u32x8/セル) b7=布頂点(float12個/頂点=48B)
//   b8=スキン済み頂点 b9=目標の更新対応(u32x2/件: 目標拘束添字・スキン頂点添字) b10=ラグランジュ乗数(float/拘束。距離拘束の後ろに目標拘束、その後ろに曲げ拘束が続く。刻みの開始で0へ戻す)
//   b11=目標拘束の引数(u32+float/拘束: 粒子添字・コンプライアンス) b12=目標位置(float4/拘束: xyz=目標位置, w=有効(1.0)か無効(0.0))
//   b13=曲げ拘束の引数(uint4+float2/拘束: 辺a・辺b・翼c・翼dの添字・静止角・曲げのコンプライアンス。彩色で色ごとに並べ替えた順。乗数は目標拘束の後ろに続く)
// 束縛番号の正本は`crates/blitz_render/src/vulkan/cloth/descriptor.rs`、拘束と乗数と目標拘束のレイアウトの正本は`crates/blitz_sim/src/gpu_layout/xpbd.rs`である。
// 介入レイアウト(blitz_sim::interventionの仕様と一致): 種別u32(0=目標拘束を有効にする,1=目標拘束を無効にする,2=インパルス)+
// 目標拘束添字u32+パディングu32x2+float4(有効にする=目標位置xyz / インパルス=xyz位置)。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct ClothParams
{
    float4 gravityDt;    // xyz=重力加速度, w=dt
    float4 gridOrigin;   // xyz=空間グリッド原点, w=セル寸法
    float4 capsuleA;     // xyz=カプセル端点A, w=カプセル半径
    float4 capsuleB;     // xyz=カプセル端点B, w=予備
    uint4 counts;        // x=粒子数, y=一辺粒子数, z=介入件数, w=目標拘束の数
    float4 misc;         // x=粒子衝突半径, y=1÷刻み幅²(コンプライアンスから刻み依存量α̃を導く), z=速度減衰, w=予備
};

[[vk::binding(0, 0)]] ConstantBuffer<ClothParams> params;
[[vk::binding(1, 0)]] RWStructuredBuffer<float4> particles;
[[vk::binding(2, 0)]] RWStructuredBuffer<float4> previousPositions;
struct Intervention
{
    uint kind;
    uint constraintIndex;
    uint2 padding;
    float4 value;
};
[[vk::binding(3, 0)]] StructuredBuffer<Intervention> interventions;
[[vk::binding(12, 0)]] RWStructuredBuffer<float4> targets;

// 掴む=目標拘束を有効にする(種別0。目標位置を書いて有効にする)。離す=目標拘束を無効にする(種別1)。粒子の逆質量は物性のまま触らない(判断6)。
// 目標拘束の数の範囲外の添字(インパルスの「対象なし」を含む)は何もしない。
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
    if (item.constraintIndex >= params.counts.w)
    {
        return;
    }
    if (item.kind == 0)
    {
        targets[item.constraintIndex] = float4(item.value.xyz, 1.0);
    }
    else if (item.kind == 1)
    {
        targets[item.constraintIndex].w = 0.0;
    }
}

// 半陰的オイラー: 速度に重力を積み、前位置を保存して位置を進める。固定は目標拘束が反復の中で行うため、全粒子を進める。
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
    float3 velocity = particles[index * 2 + 1].xyz + params.gravityDt.xyz * params.gravityDt.w;
    particles[index * 2 + 1].xyz = velocity;
    particles[index * 2].xyz = positionInvMass.xyz + velocity * params.gravityDt.w;
}
