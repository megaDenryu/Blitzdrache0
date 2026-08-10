// 点光源の影の投影の契約の写し。受け取るのは灯から表面へ向かうベクトルと影響半径、返すのは立方体の影を
// 比較サンプリングするための深度である。
// 注意: この式は`crates/blitz_render/src/point_light_shadow/projection_contract.rs`と
// `major_axis_distance.rs`の写しである。正本はRust側にあり、近面で0・遠面で1になることと、
// 面ごとの投影が書く深度と一致することのユニットテストもそちらが持つ。ここが正本から外れると、
// 影が面ごとにずれるか、接触部が浮くかのどちらかになる。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 立方体の1つの面の近面(メートル)。CPU正本の`面の近面のメートル`と同じ値でなければならない。
static const float pointLightShadowNearPlaneMeters = 0.05;

// 灯から表面へ向かうベクトルの、絶対値が最大の成分の大きさ。灯を中心とした立方体では、この値がそのまま
// そのベクトルが当たる面の透視投影の奥行きになる。
float pointLightShadowMajorAxisDistance(float3 lightToSurface)
{
    float3 magnitude = abs(lightToSurface);
    return max(magnitude.x, max(magnitude.y, magnitude.z));
}

// 主軸の距離から、シャドウマップの深度と比べるための深度を作る。近面で0、遠面で1になる順Zの値である。
// 近面より手前の距離では負の値になる。負の値は必ず比較に通るため、灯のすぐそばの表面は影に落ちない。
float pointLightShadowComparisonDepth(float majorAxisDistance, float influenceRadius)
{
    float near = pointLightShadowNearPlaneMeters;
    return influenceRadius * (majorAxisDistance - near) / (majorAxisDistance * (influenceRadius - near));
}
