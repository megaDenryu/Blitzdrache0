// 時間再構成のパス。動きベクトルで写した前のフレームの色を今のフレームの色へ混ぜ、結果を2枚へ書く。
// 第0の添付はHDR中間画像であり、以後の光のにじみ・自動露出の集計・明るさの圧縮がこの結果を読む。
// 第1の添付は履歴画像の書き込み側であり、第4成分へ今のフレームのビュー空間の奥行きを載せて次のフレームの
// 棄却の照合に使う(追加の資源を持たないため。参照: `_doc/設計/時間再構成.md`「判断f: 純関数層の中身」)。
//
// 今のフレームの色を整数の位置で読み出すのは、履歴を無効化した直後の1フレーム目の結果が今のフレームの色と
// 画素の値まで一致しなければならないためである。標本器を通すと補間の丸めがこの一致を壊しうる。
// 履歴だけは写し戻した位置が画素の中心から外れるため、線形補間の標本器で読む。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import temporal_reconstruction_blend;
import temporal_reconstruction_setting;

[[vk::binding(0, 0)]] Texture2D<float4> currentColorImage;
[[vk::binding(1, 0)]] Sampler2D historyImage;
[[vk::binding(2, 0)]] Texture2D<float2> motionVectorImage;
[[vk::binding(3, 0)]] Texture2D<float> depthImage;
[[vk::push_constant]] TemporalReconstructionSetting temporalSetting;

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

struct ReconstructionOutput
{
    float4 resolvedColor : SV_Target0;
    float4 historyRecord : SV_Target1;
};

// その画素を中心とする3x3の近傍が成分ごとに取る最小と最大。範囲の外へはみ出す位置は端の画素へ張り付ける。
// 張り付けるのは、画面の縁で近傍の件数が減ると範囲が狭まり、縁だけ履歴の寄与が落ちるためである。
void neighborhoodRange(uint2 pixel, out float3 minimum, out float3 maximum)
{
    uint2 extent = temporalSetting.extent;
    float3 center = currentColorImage.Load(int3(int(pixel.x), int(pixel.y), 0)).rgb;
    minimum = center;
    maximum = center;
    for (int down = -1; down <= 1; ++down)
    {
        for (int right = -1; right <= 1; ++right)
        {
            int2 position = int2(
                clamp(int(pixel.x) + right, 0, int(extent.x) - 1),
                clamp(int(pixel.y) + down, 0, int(extent.y) - 1));
            float3 neighbor = currentColorImage.Load(int3(position, 0)).rgb;
            minimum = min(minimum, neighbor);
            maximum = max(maximum, neighbor);
        }
    }
}

[shader("fragment")]
ReconstructionOutput fragmentMain(VertexOutput input)
{
    uint2 pixel = uint2(input.position.xy);
    int3 loadPosition = int3(int(pixel.x), int(pixel.y), 0);
    float3 currentColor = currentColorImage.Load(loadPosition).rgb;
    float currentViewDepth = temporalViewDepthOfDepth(temporalSetting, depthImage.Load(loadPosition));

    float2 motion = motionVectorImage.Load(loadPosition);
    float2 previousNdc = temporalPixelCenterNdc(temporalSetting.extent, pixel) + motion;
    float4 history = historyImage.SampleLevel(temporalNdcToUv(previousNdc), 0.0);
    bool admitted = temporalSetting.historyValid != 0
        && temporalInsideScreen(previousNdc)
        && temporalSameSurface(temporalSetting, history.a, currentViewDepth);

    float3 neighborhoodMinimum;
    float3 neighborhoodMaximum;
    neighborhoodRange(pixel, neighborhoodMinimum, neighborhoodMaximum);
    float3 resolved = temporalReconstructPixel(
        currentColor, history.rgb, neighborhoodMinimum, neighborhoodMaximum, admitted, temporalSetting.currentFrameRatio);

    ReconstructionOutput output;
    output.resolvedColor = float4(resolved, 1.0);
    output.historyRecord = float4(resolved, currentViewDepth);
    return output;
}
