// XPBDの並列方式の計測(Issue #35)の一刻みの共通工程: 予測(積分)と拘束ごとのラグランジュ乗数の零化。
// 計測の全シェーダー共通のバインディング表(統一ディスクリプタセット。各ファイルは使う分だけ宣言する):
//   b0=計測定数UBO b1=点(float4/点: xyz=位置, w=逆質量) b2=前の位置(float4/点) b3=拘束の引数(uint2+float2/拘束)
//   b4=ラグランジュ乗数(float/拘束) b5=補正の累積(int×3/点、原子加算の方式) b6=補正の候補(float4×2/拘束、二段階の方式)
//   b7=隣接の区間の開始(uint/点+1) b8=隣接の項目(uint/拘束×2)
// レイアウトの正本は `crates/blitz_sim/src/gpu_layout/xpbd.rs`、束縛番号の正本は
// `crates/blitz_render/src/vulkan/xpbd_bench/descriptor.rs` である。
// 速度は位置と前の位置の差で表す(ベルレ形)。予測は 位置 + (位置 − 前の位置) + 加速度×刻み幅² であり、
// CPUの参照計算 `crates/blitz_sim/src/constraint_graph/reference/step.rs` と同じ式である。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct XpbdParams
{
    float4 accelerationDt2; // xyz=(重力+外力)×刻み幅², w=1÷刻み幅²(コンプライアンスから刻み依存量を導く)
    uint4 counts;           // x=点の数, y=拘束の数, z=予備, w=予備
    float4 misc;            // x=固定小数の尺度(原子加算の方式の量子化), yzw=予備
};

[[vk::binding(0, 0)]] ConstantBuffer<XpbdParams> params;
[[vk::binding(1, 0)]] RWStructuredBuffer<float4> points;
[[vk::binding(2, 0)]] RWStructuredBuffer<float4> previousPositions;
[[vk::binding(4, 0)]] RWStructuredBuffer<float> lambdas;

static const uint threadsPerGroup = 64;

[shader("compute")]
[numthreads(64, 1, 1)]
void integrateMain(uint3 id: SV_DispatchThreadID)
{
    uint index = id.x;
    if (index >= params.counts.x)
    {
        return;
    }
    float4 positionInvMass = points[index];
    float3 previous = previousPositions[index].xyz;
    previousPositions[index] = float4(positionInvMass.xyz, 0.0);
    if (positionInvMass.w == 0.0)
    {
        return;
    }
    float3 predicted = positionInvMass.xyz + (positionInvMass.xyz - previous) + params.accelerationDt2.xyz;
    points[index] = float4(predicted, positionInvMass.w);
}

// 物理刻みの開始でラグランジュ乗数を0へ戻す(判断5)。
[shader("compute")]
[numthreads(64, 1, 1)]
void lambdaClearMain(uint3 id: SV_DispatchThreadID)
{
    uint index = id.x;
    if (index >= params.counts.y)
    {
        return;
    }
    lambdas[index] = 0.0;
}
