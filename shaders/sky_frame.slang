// 空パスの全画面三角形の頂点段と太陽円盤の見え方。放射輝度の評価(sky_atmosphere.slang)と
// 空中遠近の合成(aerial_composite.slang)がこの1つのモジュールを取り込み、視線の復元を共有する。
// 定数そのものは`sky_pass_uniform.slang`(空専用の値)と`view_uniform.slang`(カメラ相対位置)が宣言する。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。
// 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「空パスの配置」

import sky_pass_uniform;
import view_uniform;

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
    float4 farPoint = mul(skyPassUniform.inverseViewProjection, float4(ndc, 1.0, 1.0));
    output.viewRay = farPoint.xyz / farPoint.w - viewUniform.cameraRelativePosition.xyz;
    return output;
}

// 太陽円盤の見え方。角半径の内外を余弦で比べ、縁を1割の幅で滑らかにしてジャギーを避ける。
// sky_atmosphere.slangがこのモジュールを取り込んで呼ぶため、ここが唯一の正本である。
float sunDiskCoverage(float cosGamma)
{
    float sunCos = skyPassUniform.nightSkyRadianceAndSunCos.w;
    float outerCos = cos(acos(sunCos) * 1.1);
    float innerCos = cos(acos(sunCos) * 0.9);
    return smoothstep(outerCos, innerCos, cosGamma);
}
