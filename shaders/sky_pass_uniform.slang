// 空パス定数の唯一の宣言。空パスと空中遠近の合成パスだけがこの定数を読む。
// 空段階を持たないフレーム構成ではCPU側がこのバッファを1度も書かない
// (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「分離の形」)。
// CPU側の正本は`crates/blitz_render/src/vulkan/uniform/sky_bytes.rs`であり、並びの一致を単体テストが見る。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

struct SkyPassUniform
{
    // クリップ座標からカメラ相対ワールドへ戻す変換。全画面三角形の頂点から視線の向きを復元するために使う。
    column_major float4x4 inverseViewProjection;
    // rgbが夜間の背景放射輝度、wが太陽円盤の角半径の余弦。
    float4 nightSkyRadianceAndSunCos;
    // xyzが地表から太陽へ向かう単位ベクトル(Xが東・Yが天頂・Zが南)、wがスカイビューのベイク済み画像を焼いた観測高度(メートル)。
    float4 sunDirectionAndObserverAltitude;
    // rgbが太陽円盤の放射輝度、wが太陽の放射照度の尺度。
    float4 sunDiskRadianceAndSunIrradiance;
};

[[vk::binding(2, 0)]]
ConstantBuffer<SkyPassUniform> skyPassUniform;
