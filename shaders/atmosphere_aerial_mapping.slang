// 空中遠近ボリュームのボクセル座標と、画面の位置およびカメラからの距離との行き来。
// 横と縦は画面を等分した区画であり、焼く側はその中心を使い、引く側は画素の画面位置をそのまま渡す。
// 奥行きは距離を線形に刻み、先頭のスライスを距離0に置く。
// 注意: この式は`crates/blitz_render/src/atmosphere/mapping/aerial_mapping.rs`の写しである。
// 正本はRust側にあり、ずれると焼いた座標と引く座標が食い違って霞の量が静かにずれる。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// カメラの視錐台を、画面の単位位置から視線の向きを与えるアフィン写像として表したもの。
// 遠平面上の点はクリップ座標のwが一定であるため、この写像はアフィンであり基準1つと差分2つで厳密に表せる。
struct AerialFrustumRays
{
    float3 baseDirection;
    float3 horizontalDelta;
    float3 verticalDelta;
};

// 画面の単位位置に対応する視線の単位ベクトル。
float3 aerialViewDirection(AerialFrustumRays rays, float2 screenPosition)
{
    float3 direction = rays.baseDirection + rays.horizontalDelta * screenPosition.x + rays.verticalDelta * screenPosition.y;
    return normalize(direction);
}

// ボクセルの横縦の添字から、そのボクセルが受け持つ画面の区画の中心の位置を求める。
float2 aerialScreenPositionOfVoxel(uint2 texel, uint side)
{
    return (float2(texel) + 0.5) / float(side);
}

// 奥行きのボクセルの添字から、そのボクセルが表す奥行きの単位位置を求める。
// 両端の添字がちょうど0と1に対応するため、先頭のスライスが距離0、末尾のスライスが最遠距離になる。
float aerialDepthUnitOfVoxel(uint slice, uint side)
{
    return float(slice) / float(side - 1);
}

// カメラからの距離から奥行きの単位位置を求める。最遠距離より遠い距離は1へ張り付ける。
float aerialDepthUnitOfDistance(float farDistance, float distance)
{
    return farDistance <= 0.0 ? 0.0 : clamp(distance / farDistance, 0.0, 1.0);
}

// 引く側がボリュームを標本する3成分の座標。横縦は画面の位置をそのまま渡し、奥行きは単位位置をテクセル中心のUVへ直す。
// 横縦が素通りなのは、焼く側が区画の中心を標本点に取るため画面の位置がそのままテクセル中心のUVになるからである。
float3 aerialSampleCoordinate(float2 screenPosition, float depthUnit, uint side)
{
    float depthUv = (depthUnit * float(side - 1) + 0.5) / float(side);
    return float3(screenPosition, depthUv);
}
