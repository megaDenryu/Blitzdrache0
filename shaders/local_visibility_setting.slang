// 局所可視性補正の2つのコンピュートが共有する即時定数と、そこから導く3つの規則(奥行きの差の下限・
// 同じ面と認める距離の上限・奥行きの差に対する遮蔽の重み)。
// 2つのエントリが同じ並びを押し込むのは、遮蔽の標本化とぼかしが同じ射影と同じ寸法を読むためである。
// 片方だけ別の並びにすると、同じ画面を別の射影で解釈した2枚が重なる。
// 注意: 並びと値の意味は`crates/blitz_render/src/local_visibility/occlusion_settings.rs`と
// `blur_settings.rs`・`depth_bias.rs`・`distance_falloff.rs`・`same_surface_tolerance.rs`の写しである。
// 正本はRust側にあり、CPU側の押し込みの並びは`crates/blitz_render/src/vulkan/local_visibility/setting.rs`が持つ。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import local_visibility_projection;

struct LocalVisibilitySetting
{
    LocalVisibilityProjection projection;
    uint2 extent;
    // 遮蔽を探しに行く半球の半径(ビュー空間のメートル)。
    float occlusionRadius;
    // 遮蔽と判定するために要る奥行きの差の下限を、1画素が覆う幅への倍率で表した値。
    float depthBiasScale;
    // 遮蔽の重みを丸ごと保つ距離と、重みが尽きる距離。どちらも半径への倍率である。
    float falloffHoldScale;
    float falloffEndScale;
    // 1画素あたりに置く遮蔽の標本の本数。
    uint sampleCount;
    // 法線の再構成で同じ面に載ると認める距離の上限の、1画素が覆う幅への倍率。
    float occlusionSameSurfaceTolerance;
    // 両側ぼかしの窓の半径(画素)と、窓の中で同じ面に載ると認める距離の上限の倍率。
    uint blurPixelRadius;
    float blurSameSurfaceTolerance;
};

// 標本1件が遮蔽の割合へ寄与しうる最大の重み。全件が遮蔽なら割合はちょうど1になる。
float localVisibilitySampleWeight(uint sampleCount)
{
    return 1.0 / float(sampleCount);
}

// その奥行きでの奥行きの差の下限。標点の載る面そのものが標本の背後に僅かに現れるため、下限を置かないと
// 平坦な面が自分自身を遮蔽して縞が出る。
float localVisibilityDepthBias(float pixelCoverage, float depthBiasScale)
{
    return pixelCoverage * depthBiasScale;
}

// その奥行きで、同じ面に載っていると認める距離の上限。
float localVisibilitySameSurfaceLimit(float pixelCoverage, float toleranceScale)
{
    return pixelCoverage * toleranceScale;
}

// 遮蔽物と標点の奥行きの差に対する遮蔽の重み。差が小さいうちは1、尽きる距離を越えると0であり、
// 間は両端で傾きが0になる三次の曲線でつなぐ。断ち切りを段でなく滑らかにするのは、重みが跳ぶと
// 物体の輪郭に沿って濃さの段差が出るためである。
float localVisibilityFalloffWeight(LocalVisibilitySetting setting, float depthDifference)
{
    float difference = abs(depthDifference);
    float holdDistance = setting.occlusionRadius * setting.falloffHoldScale;
    float endDistance = setting.occlusionRadius * setting.falloffEndScale;
    if (!isfinite(difference) || difference >= endDistance)
    {
        return 0.0;
    }
    if (difference <= holdDistance)
    {
        return 1.0;
    }
    float progress = (difference - holdDistance) / (endDistance - holdDistance);
    return 1.0 - progress * progress * (3.0 - 2.0 * progress);
}
