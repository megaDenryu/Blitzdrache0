// M7: ブルームパス(判断38・39)。HDR中間画像から閾値1.0超の輝度成分を1/2解像度へ抽出し、
// 分離ガウシアンでぼかす。横/縦は同一エントリを方向プッシュ定数で使い分ける。
// 合成はトーンマップパス(tonemap.slang)が行う。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct BlurPush
{
    // ぼかし方向(テクセル単位)。横=(1,0)、縦=(0,1)。
    float2 direction;
};

[[vk::push_constant]]
BlurPush blurPush;

[[vk::binding(0, 0)]] Sampler2D sourceTexture;

struct VertexOutput
{
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
};

[shader("vertex")]
VertexOutput vertexMain(uint vertexID : SV_VertexID)
{
    // 全画面三角形(tonemap.slangと同じcorner写像)。
    float2 corner = float2(float((vertexID << 1) & 2), float(vertexID & 2));
    VertexOutput output;
    output.position = float4(corner * 2.0 - 1.0, 0.0, 1.0);
    output.uv = corner;
    return output;
}

// 閾値1.0を超える成分だけを通す。輝度でのハードカットでなく成分ごとの減算に
// しているのは、閾値境界のちらつきを避けて滑らかに立ち上げるため。
[shader("fragment")]
float4 extractMain(VertexOutput input) : SV_Target
{
    float3 hdr = sourceTexture.Sample(input.uv).rgb;
    return float4(max(hdr - 1.0, 0.0), 1.0);
}

// 9タップ分離ガウシアン(片側4タップ+中心)。重みは正規化済み定数。
[shader("fragment")]
float4 blurMain(VertexOutput input) : SV_Target
{
    float weights[5] = { 0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216 };
    uint width;
    uint height;
    sourceTexture.GetDimensions(width, height);
    float2 texelSize = float2(1.0 / float(width), 1.0 / float(height));
    float2 step = blurPush.direction * texelSize;

    float3 sum = sourceTexture.Sample(input.uv).rgb * weights[0];
    for (int i = 1; i < 5; i++)
    {
        float2 offset = step * float(i);
        sum += sourceTexture.Sample(input.uv + offset).rgb * weights[i];
        sum += sourceTexture.Sample(input.uv - offset).rgb * weights[i];
    }
    return float4(sum, 1.0);
}
