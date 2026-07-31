// 光のにじみピラミッドのフィルタ関数群(判断41)。縮小は13タップ(CoD:Advanced Warfare方式)、
// 拡大は3x3テント。前処理用にKaris平均とソフト閾値を持つ。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 13タップの5グループ(バイリニア2x2グループ)。groups[0]=中央、[1..4]=四隅。
void downsampleGroups(Sampler2D tex, float2 uv, float2 texel, out float3 groups[5])
{
    float3 a = tex.Sample(uv + texel * float2(-2.0, -2.0)).rgb;
    float3 b = tex.Sample(uv + texel * float2(0.0, -2.0)).rgb;
    float3 c = tex.Sample(uv + texel * float2(2.0, -2.0)).rgb;
    float3 d = tex.Sample(uv + texel * float2(-2.0, 0.0)).rgb;
    float3 e = tex.Sample(uv).rgb;
    float3 f = tex.Sample(uv + texel * float2(2.0, 0.0)).rgb;
    float3 g = tex.Sample(uv + texel * float2(-2.0, 2.0)).rgb;
    float3 h = tex.Sample(uv + texel * float2(0.0, 2.0)).rgb;
    float3 i = tex.Sample(uv + texel * float2(2.0, 2.0)).rgb;
    float3 j = tex.Sample(uv + texel * float2(-1.0, -1.0)).rgb;
    float3 k = tex.Sample(uv + texel * float2(1.0, -1.0)).rgb;
    float3 l = tex.Sample(uv + texel * float2(-1.0, 1.0)).rgb;
    float3 m = tex.Sample(uv + texel * float2(1.0, 1.0)).rgb;
    groups[0] = (j + k + l + m) * 0.25;
    groups[1] = (a + b + d + e) * 0.25;
    groups[2] = (b + c + e + f) * 0.25;
    groups[3] = (d + e + g + h) * 0.25;
    groups[4] = (e + f + h + i) * 0.25;
}

// 13タップ縮小。中央グループ0.5、四隅グループ0.125ずつの重み。
float3 downsample13(Sampler2D tex, float2 uv, float2 texel)
{
    float3 groups[5];
    downsampleGroups(tex, uv, texel, groups);
    return groups[0] * 0.5 + (groups[1] + groups[2] + groups[3] + groups[4]) * 0.125;
}

// Karis平均の重み: 輝度の逆数。1画素だけ極端に明るいグループの寄与を抑える(ファイアフライ対策)。
float karisWeight(float3 groupAverage)
{
    float luma = dot(groupAverage, float3(0.2126, 0.7152, 0.0722));
    return 1.0 / (1.0 + luma);
}

// 13タップ縮小のKaris平均版(縮小チェーン初段=前処理でのみ使う)。
float3 downsample13Karis(Sampler2D tex, float2 uv, float2 texel)
{
    float3 groups[5];
    downsampleGroups(tex, uv, texel, groups);
    float w0 = karisWeight(groups[0]) * 0.5;
    float w1 = karisWeight(groups[1]) * 0.125;
    float w2 = karisWeight(groups[2]) * 0.125;
    float w3 = karisWeight(groups[3]) * 0.125;
    float w4 = karisWeight(groups[4]) * 0.125;
    float3 sum = groups[0] * w0 + groups[1] * w1 + groups[2] * w2 + groups[3] * w3 + groups[4] * w4;
    return sum / (w0 + w1 + w2 + w3 + w4);
}

// 3x3テントフィルタ(拡大チェーン用)。texelは読み元(1段小さい解像度)の1テクセル。
float3 tent9(Sampler2D tex, float2 uv, float2 texel)
{
    float3 sum = tex.Sample(uv).rgb * 4.0;
    sum += tex.Sample(uv + texel * float2(-1.0, 0.0)).rgb * 2.0;
    sum += tex.Sample(uv + texel * float2(1.0, 0.0)).rgb * 2.0;
    sum += tex.Sample(uv + texel * float2(0.0, -1.0)).rgb * 2.0;
    sum += tex.Sample(uv + texel * float2(0.0, 1.0)).rgb * 2.0;
    sum += tex.Sample(uv + texel * float2(-1.0, -1.0)).rgb;
    sum += tex.Sample(uv + texel * float2(1.0, -1.0)).rgb;
    sum += tex.Sample(uv + texel * float2(-1.0, 1.0)).rgb;
    sum += tex.Sample(uv + texel * float2(1.0, 1.0)).rgb;
    return sum / 16.0;
}

// ソフト閾値(閾値1.0・ニー0.5)。閾値付近を二次カーブで滑らかにつなぎ、
// ハードカットの境界ちらつきを防ぐ(判断41)。
float3 softThreshold(float3 color)
{
    float threshold = 1.0;
    float knee = 0.5;
    float brightness = max(color.r, max(color.g, color.b));
    float soft = clamp(brightness - threshold + knee, 0.0, 2.0 * knee);
    soft = soft * soft / (4.0 * knee + 1e-5);
    float contribution = max(soft, brightness - threshold) / max(brightness, 1e-5);
    return color * contribution;
}
