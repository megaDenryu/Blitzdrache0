// 空中遠近の全画面合成。深度がジオメトリの画素だけへ、空中遠近ボリュームから参照した散乱を足し、
// もとのシーン色へ代表透過率を掛ける。掛け算はパイプラインのブレンド(src係数ONE・dst係数SRC_ALPHA)が行うため、
// 画素段が返すのはRGBに太陽の放射照度を掛けた散乱、Aに代表透過率である。シーン色をテクスチャで読み直さない。
// 深度がクリア値のままの画素を捨てるのは、そこを空パスが描くためである。両方が霞ませると空が二重に霞む。
// 視線の向きも最遠距離も焼く側と同じ写像を通す。写像の正本は`crates/blitz_render/src/atmosphere/mapping/aerial_mapping.rs`であり、
// 写しは`atmosphere_aerial_mapping`が持つ。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。
// 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「空中遠近ボリュームの刻みと積分」

import atmosphere_aerial_mapping;
import sky_frame;

// set1は合成パスだけが束縛するディスクリプタである。深度はNEARESTのサンプラーを結ぶ。輪郭で深度を混ぜると、
// 手前の面と奥の面の間の距離が出てそこだけ霞の量が跳ぶ。ボリュームはLINEARであり、3軸の補間が霞を滑らかにする。
// ボリュームのレイアウトはGENERALであり、レンダーグラフの画像用途「大気のベイク済み画像の画素段参照」が同じレイアウトを指す。
[[vk::binding(0, 1)]] Sampler2D sceneDepth;
[[vk::binding(1, 1)]] Sampler3D aerialVolume;

// そのフレームのボリュームの最遠距離。フレームシェーダー定数へ載せず即時定数で渡すのは、この値の出どころが
// 焼く側と同じ空中遠近観測条件の1箇所であり、シェーダー定数へ写すと焼いた刻みと参照する刻みが食い違いうるためである。
struct AerialCompositeCondition
{
    float farDistance;
};
[[vk::push_constant]] AerialCompositeCondition compositeCondition;

// 画面の単位位置と深度から、カメラ相対フレームでのその画素の点を求める。カメラ位置を参照するため、返る量は
// カメラからその面へのベクトルであり、その長さが焼く側の距離の目盛りと同じユークリッド距離である。
float3 cameraRelativePointOfDepth(float2 screenPosition, float depth)
{
    float2 ndc = screenPosition * 2.0 - 1.0;
    float4 surfacePoint = mul(frameUniform.inverseViewProjection, float4(ndc, depth, 1.0));
    return surfacePoint.xyz / surfacePoint.w - frameUniform.cameraRelativePosition.xyz;
}

[shader("fragment")]
float4 fragmentMain(VertexOutput input) : SV_Target
{
    uint depthWidth;
    uint depthHeight;
    sceneDepth.GetDimensions(depthWidth, depthHeight);
    // 深度バッファは描画先と1対1であるため、画素の中心がそのままテクセルの中心であり画面の単位位置でもある。
    float2 screenPosition = input.position.xy / float2(float(depthWidth), float(depthHeight));
    float depth = sceneDepth.SampleLevel(screenPosition, 0.0).x;
    if (depth >= 1.0)
    {
        // 返す値はブレンドの恒等元(散乱0・透過率1)である。discardが効かない経路でも絵が変わらない。
        discard;
        return float4(0.0, 0.0, 0.0, 1.0);
    }

    float3 cameraRelative = cameraRelativePointOfDepth(screenPosition, depth);
    float depthUnit = aerialDepthUnitOfDistance(compositeCondition.farDistance, length(cameraRelative));
    uint volumeWidth;
    uint volumeHeight;
    uint volumeDepth;
    aerialVolume.GetDimensions(volumeWidth, volumeHeight, volumeDepth);
    float4 voxel = aerialVolume.SampleLevel(aerialSampleCoordinate(screenPosition, depthUnit, volumeDepth), 0.0);

    float sunIrradiance = frameUniform.sunDiskRadianceAndSunIrradiance.w;
    return float4(max(voxel.rgb * sunIrradiance, 0.0), clamp(voxel.a, 0.0, 1.0));
}
