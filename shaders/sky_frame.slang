// 空パスが読むフレームシェーダー定数の並びと、全画面三角形の頂点段。放射輝度の評価(sky_atmosphere.slang)と
// 空中遠近の合成(aerial_composite.slang)がこの1つのモジュールを取り込み、入力の並びと視線の復元を共有する。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。
// 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「空パスの配置」

// binding3のフレームシェーダー定数。前半はscene.slangのFrameUniformと同じ並びであり、後半の inverseViewProjection 以降が空パスだけが読む区画である。
// 注意: 前半の並びをscene.slangと食い違わせると後半の開始位置がずれる。区画の書き手は
// `crates/blitz_render/src/vulkan/uniform/sky_bytes.rs`であり、並びを変えるときは両方を同時に直す。
struct SkyFrameUniform
{
    column_major float4x4 cascadeLightViewProjection[4];
    column_major float4x4 viewProjection;
    float4 cameraRelativePosition;
    float4 directionalLightDirection;
    float4 directionalLightColor;
    float4 pointLightPosition;
    float4 pointLightColor;
    float4 ambientAndLighting;
    float4 cascadeSplitDepths;
    float4 cascadeTransitionWidths;
    float4 cameraForward;
    column_major float4x4 inverseViewProjection;
    // rgbが夜間の背景放射輝度、wが太陽円盤の角半径の余弦。
    float4 nightSkyRadianceAndSunCos;
    // xyzが地表から太陽へ向かう単位ベクトル(Xが東・Yが天頂・Zが南)、wがスカイビューのベイク済み画像を焼いた観測高度(メートル)。
    float4 sunDirectionAndObserverAltitude;
    // rgbが太陽円盤の放射輝度、wが太陽の放射照度の尺度。
    float4 sunDiskRadianceAndSunIrradiance;
};

[[vk::binding(3, 0)]]
ConstantBuffer<SkyFrameUniform> frameUniform;

struct VertexOutput
{
    float4 position : SV_Position;
    // カメラ相対フレームでの視線ベクトル。全画面三角形はwが一定のため、頂点間の線形補間が正確な向きを与える。
    float3 viewRay : TEXCOORD0;
};

[shader("vertex")]
VertexOutput vertexMain(uint vertexID : SV_VertexID)
{
    // 全画面三角形: 頂点ID 0,1,2 をクリップ座標 (-1,-1),(3,-1),(-1,3) に写す。
    // 深度は遠平面ちょうど(z=w=1)に置く。深度クリア値と同じ値であり、パイプラインのEQUAL比較が未描画画素だけを選ぶ。
    float2 corner = float2(float((vertexID << 1) & 2), float(vertexID & 2));
    float2 ndc = corner * 2.0 - 1.0;
    VertexOutput output;
    output.position = float4(ndc, 1.0, 1.0);
    float4 farPoint = mul(frameUniform.inverseViewProjection, float4(ndc, 1.0, 1.0));
    output.viewRay = farPoint.xyz / farPoint.w - frameUniform.cameraRelativePosition.xyz;
    return output;
}

// 太陽円盤の見え方。角半径の内外を余弦で比べ、縁を1割の幅で滑らかにしてジャギーを避ける。
// sky_atmosphere.slangがこのモジュールを取り込んで呼ぶため、ここが唯一の正本である。
float sunDiskCoverage(float cosGamma)
{
    float sunCos = frameUniform.nightSkyRadianceAndSunCos.w;
    float outerCos = cos(acos(sunCos) * 1.1);
    float innerCos = cos(acos(sunCos) * 0.9);
    return smoothstep(outerCos, innerCos, cosGamma);
}
