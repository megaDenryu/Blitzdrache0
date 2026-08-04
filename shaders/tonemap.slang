// M7: 明るさの圧縮パス(判断38・39)。HDR中間画像(リニア)を全画面三角形でサンプリングし、
// 光のにじみ(1/2解像度)を加算合成してから露出乗算とACES近似で表示リニア0..1へ圧縮する。
// スワップチェーンはSRGB形式のため、sRGBへの符号化はハードウェアが行う。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import auto_exposure_state;

// 露出の決め方。`automatic`が0の枝では`exposure`がそのまま最終倍率であり、時刻別固定の補正段はCPUが畳み込み済みである。
// 1の枝では基準倍率へ「芸術的バイアス+GPU上の自動補正段」を2の冪として掛ける。自動補正段をCPUのプッシュ定数で
// 渡さないのは、そうすると同じフレームの集計が次のフレームの絵にしか効かず、1フレーム遅れになるためである。
struct ExposurePush
{
    float exposure;
    float artisticBiasStep;
    uint automatic;
};

[[vk::push_constant]]
ExposurePush exposurePush;

[[vk::binding(0, 0)]] Sampler2D hdrTexture;
[[vk::binding(1, 0)]] Sampler2D bloomTexture;
[[vk::binding(2, 0)]] StructuredBuffer<ExposureState> exposureState;

// そのフレームの最終露出倍率。
float finalExposure()
{
    if (exposurePush.automatic == 0)
    {
        return exposurePush.exposure;
    }
    float step = exposurePush.artisticBiasStep + automaticCompensationStep(exposureState[0]);
    return exposurePush.exposure * exp2(step);
}

struct VertexOutput
{
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
};

[shader("vertex")]
VertexOutput vertexMain(uint vertexID : SV_VertexID)
{
    // 全画面三角形: 頂点ID 0,1,2 をクリップ座標 (-1,-1),(3,-1),(-1,3) に写す。
    // VulkanのNDCはY下向きのため、uv=(0,0)が画面左上・テクスチャ先頭行と一致する。
    float2 corner = float2(float((vertexID << 1) & 2), float(vertexID & 2));
    VertexOutput output;
    output.position = float4(corner * 2.0 - 1.0, 0.0, 1.0);
    output.uv = corner;
    return output;
}

// ACESフィルミックトーンカーブの近似(Narkowiczの定数)。入力はリニアHDR、出力は0..1。
float3 acesFilm(float3 x)
{
    float a = 2.51;
    float b = 0.03;
    float c = 2.43;
    float d = 0.59;
    float e = 0.14;
    return saturate((x * (a * x + b)) / (x * (c * x + d) + e));
}

// ピラミッド結果に掛ける合成強度(判断41)。拡大チェーンは段ごとに加算して値が積み上がるため
// 小さい係数で戻す。厳密なエネルギー保存(元からの減算)は行わず、ソフト閾値+小強度で過剰を避ける。
static const float bloomStrength = 0.06;

[shader("fragment")]
float4 fragmentMain(VertexOutput input) : SV_Target
{
    // 光のにじみはリニアHDRのまま加算し、露出・トーンカーブは合成後の値に一度だけ掛ける(判断38の色の流れ)。
    float3 hdr = hdrTexture.Sample(input.uv).rgb + bloomTexture.Sample(input.uv).rgb * bloomStrength;
    float3 exposed = hdr * finalExposure();
    return float4(acesFilm(exposed), 1.0);
}
