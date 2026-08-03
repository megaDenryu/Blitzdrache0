// 太陽相対座標のフレームの組み立てと、ワールドの向きとの行き来。
// 遠方環境をワールド軸でなく太陽相対座標で焼くのは、鍵に太陽の方位を入れずに済ませるためである。
// 第一軸は太陽の水平成分の向き、第二軸は天頂(+Y)、第三軸は第一軸と第二軸の外積である。
// 注意: この写像は`crates/blitz_render/src/atmosphere/mapping/sun_relative_frame.rs`の写しである。正本はRust側にある。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 太陽の水平成分がこの長さ未満なら方位を決められないとみなす。測るのは`length`(ユークリッドの長さ)であり、
// 正本の`方位を決められない水平成分の長さ`と同じノルムである。
// 正本は`crates/blitz_render/src/atmosphere/mapping/skyview_lookup.rs`にある。
static const float sunRelativeAzimuthEpsilon = 1.0e-6;

struct SunRelativeFrame
{
    float3 firstAxis;
    float3 secondAxis;
    float3 thirdAxis;
};

// 太陽方向からフレームを組む。太陽方向は単位ベクトルであり、天頂が+Yである。
// 天頂または天底にある太陽はどの方位から見ても対称であるため、方位を決められないときは第一軸を+Xへ固定する。
SunRelativeFrame sunRelativeFrameOf(float3 sunDirection)
{
    float2 horizontal = float2(sunDirection.x, sunDirection.z);
    float3 firstAxis = float3(1.0, 0.0, 0.0);
    if (length(horizontal) >= sunRelativeAzimuthEpsilon)
    {
        float2 unitHorizontal = normalize(horizontal);
        firstAxis = float3(unitHorizontal.x, 0.0, unitHorizontal.y);
    }

    SunRelativeFrame frame;
    frame.firstAxis = firstAxis;
    frame.secondAxis = float3(0.0, 1.0, 0.0);
    frame.thirdAxis = float3(-firstAxis.z, 0.0, firstAxis.x);
    return frame;
}

// ワールドの向きを太陽相対座標へ写す。3本の軸との内積を並べるだけである。
float3 sunRelativeOfWorld(SunRelativeFrame frame, float3 direction)
{
    return float3(dot(direction, frame.firstAxis), dot(direction, frame.secondAxis), dot(direction, frame.thirdAxis));
}

// 太陽の天頂余弦から、太陽相対座標での太陽の向きを求める。第三軸の成分は定義から必ず0である。
float3 sunRelativeSunDirection(float sunZenithCosine)
{
    float lateral = sqrt(max(0.0, 1.0 - sunZenithCosine * sunZenithCosine));
    return float3(lateral, sunZenithCosine, 0.0);
}
