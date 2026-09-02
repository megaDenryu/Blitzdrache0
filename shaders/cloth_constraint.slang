// M9/Issue #36: 布の構造とせん断の距離拘束を本来のXPBD(コンプライアンスとラグランジュ乗数を持つ正典式)で解く工程。
// 並列方式はグラフ彩色(判断7の採用規則1): 同じ粒子を触らない拘束の集合(色)ごとに1回のディスパッチを積み、
// 1色の中では拘束1本につき1スレッドが乗数と両端の位置を直接書く。色の区間はプッシュ定数で受ける。
// 色の中で粒子を共有しないことをCPUの彩色(`blitz_sim::布の彩色済み拘束`)が保証するため、原子演算も平均も要らず、
// 逐次(ガウス・ザイデル型)の更新であるため緩和係数も掛けない。結果はビット単位で決定的である。
// 正典式は`xpbd_projection.slang`の1つの写しを読み、式をここへ再び書かない。退化(両方が固定・向きが定まらない)は補正0で乗数を変えない。
// 乗数は物理刻みの開始で`lambdaClearMain`が0へ戻し(判断5)、その刻みの反復の間だけ累積する。零化の区間は距離拘束と、その後ろに続く目標拘束(cloth_target.slang)の合計である。
// バインディング表と定数レイアウトはcloth_step.slang冒頭の仕様に従う(b4=拘束の引数、b10=ラグランジュ乗数)。

import xpbd_projection;

struct ClothParams
{
    float4 gravityDt;
    float4 gridOrigin;
    float4 capsuleA;
    float4 capsuleB;
    uint4 counts;
    float4 misc;
};

// 1拘束16バイト。並びは彩色で色ごとに並べ替えた順であり、`blitz_sim::gpu_layout::xpbd`の拘束の引数の契約と同じである。
struct ConstraintParams
{
    uint a;
    uint b;
    float restLength;
    float compliance;
};

// 1色の区間。並べ替えた拘束一覧の中で色が占める連続した範囲であり、CPUの`布の彩色の区間`と同じ2語である。
struct ColorRange
{
    uint start;
    uint count;
};

[[vk::binding(0, 0)]] ConstantBuffer<ClothParams> params;
[[vk::binding(1, 0)]] RWStructuredBuffer<float4> particles;
[[vk::binding(4, 0)]] StructuredBuffer<ConstraintParams> constraints;
[[vk::binding(10, 0)]] RWStructuredBuffer<float> lambdas;
[[vk::push_constant]] ColorRange colorRange;

static const uint threadsPerGroup = 64;

// 物理刻みの開始でラグランジュ乗数を0へ戻す(判断5)。拘束1本につき1スレッドであり、区間は全拘束(開始0・本数=拘束の数)を受ける。
[shader("compute")]
[numthreads(64, 1, 1)]
void lambdaClearMain(uint3 id: SV_DispatchThreadID)
{
    if (id.x >= colorRange.count)
    {
        return;
    }
    lambdas[colorRange.start + id.x] = 0.0;
}

// 1色の区間の拘束を正典式で1回射影し、乗数と両端の位置を直接書く。粒子は32バイト(位置と逆質量のfloat4 + 速度のfloat4)である。
[shader("compute")]
[numthreads(64, 1, 1)]
void constraintMain(uint3 id: SV_DispatchThreadID)
{
    if (id.x >= colorRange.count)
    {
        return;
    }
    uint index = colorRange.start + id.x;
    ConstraintParams constraint = constraints[index];
    float4 a = particles[constraint.a * 2];
    float4 b = particles[constraint.b * 2];
    float alphaTilde = constraint.compliance * params.misc.y;
    DistanceProjection projection = projectDistance(a.xyz, a.w, b.xyz, b.w, constraint.restLength, alphaTilde, lambdas[index]);
    if (!projection.corrected)
    {
        return;
    }
    lambdas[index] += projection.deltaLambda;
    particles[constraint.a * 2] = float4(a.xyz + projection.correctionA, a.w);
    particles[constraint.b * 2] = float4(b.xyz + projection.correctionB, b.w);
}
