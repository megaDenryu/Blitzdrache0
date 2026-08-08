// 空パスの全画面三角形の頂点段と太陽円盤の見え方。放射輝度の評価(sky_atmosphere.slang)と
// 空中遠近の合成(aerial_composite.slang)がこの1つのモジュールを取り込み、視線の復元を共有する。
// 動きベクトルの材料もこの頂点段が作る。書くのは空パスだけであり、空中遠近の合成は色だけを書く。
// 定数そのものは`sky_pass_uniform.slang`(空専用の値)と`view_uniform.slang`(カメラ相対位置)が宣言する。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。
// 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「空パスの配置」

import motion_vector;
import sky_pass_uniform;
import view_uniform;

struct VertexOutput
{
    float4 position : SV_Position;
    // カメラ相対フレームでの視線ベクトル。全画面三角形はwが一定のため、頂点間の線形補間が正確な向きを与える。
    float3 viewRay : TEXCOORD0;
    // 今フレームのビュー射影が視線方向を写したクリップ位置。ラスタライズの位置をそのまま使わず同じ視線から作り直すのは、
    // 視線が逆ビュー射影を1度通って復元された値だからである。前フレームの側だけが往復の誤差を負うと、静止したカメラでも
    // 動きベクトルが1画素ぶん残る(実測で最大0.002026、画面の15パーセントの画素が1画素の幅を超えた)。
    // 同じ視線を同じ向きの変換で2度写せば、その誤差は差で消える。
    float4 currentClipPosition : TEXCOORD1;
    // 前フレームのずらし無しビュー射影が同じ視線方向を写したクリップ位置。空には奥行きが無いため、
    // 無限遠の点として第4成分を0で通す。平行移動の成分が効かなくなり、カメラの回転だけが画面上の移動を作る。
    float4 previousClipPosition : TEXCOORD2;
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
    output.currentClipPosition = mul(viewUniform.viewProjection, float4(output.viewRay, 0.0));
    output.previousClipPosition = mul(viewUniform.previousViewProjection, float4(output.viewRay, 0.0));
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
