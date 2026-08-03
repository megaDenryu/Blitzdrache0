// 遠方環境から焼いた派生表現を標準PBRが消費する式。拡散照度と鏡面畳込みと反射率積分表の3つの値から、
// 表面が返す間接光の色を決める。画像の参照方法はこのモジュールが知らず、値だけを受け取る。
// 注意: この式は`crates/blitz_render/src/distant_environment/consume/`の写しである。正本はRust側にある。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 誘電体(金属でない材質)の垂直入射の反射率。
// 正本は`crates/blitz_render/src/distant_environment/consume/fresnel.rs`にある。
static const float dielectricBaseReflectance = 0.04;

// 垂直入射の反射率。金属は自分の色で反射し、誘電体は色を持たない一定の割合で反射する。
float3 indirectNormalIncidenceReflectance(float3 baseColor, float metallic)
{
    return lerp(float3(dielectricBaseReflectance, dielectricBaseReflectance, dielectricBaseReflectance), baseColor, metallic);
}

// 粗さ対応の反射率。間接光は半球全体から来るため中間の向きが1本に定まらず、法線と視線の余弦で代用する。
// 粗い表面は端で反射率が上がりきらないため、上限を粗さで下げる。
float3 indirectRoughnessFresnel(float nDotV, float3 normalIncidenceReflectance, float roughness)
{
    float decay = pow(1.0 - saturate(nDotV), 5.0);
    float3 ceiling = max(float3(1.0 - saturate(roughness)), normalIncidenceReflectance);
    return normalIncidenceReflectance + (ceiling - normalIncidenceReflectance) * decay;
}

// 拡散が受け取るエネルギーの取り分。鏡面へ跳ね返った光は表面の内側へ入らず、金属は透過した光を吸収する。
float3 indirectDiffuseRatio(float3 fresnel, float metallic)
{
    return (float3(1.0, 1.0, 1.0) - fresnel) * (1.0 - saturate(metallic));
}

// 拡散の間接光の色。拡散照度の画像が保つのはLambertの畳み込み後の照度そのものであり、円周率で割るのは消費側である。
float3 indirectDiffuseColor(float3 diffuseRatio, float3 irradiance, float3 baseColor)
{
    return diffuseRatio * irradiance * baseColor / 3.14159265;
}

// 鏡面の間接光の色。反射率積分表の第1成分が垂直入射の反射率に掛かる倍率、第2成分が反射率に依らず足す量である。
float3 indirectSpecularColor(float3 prefilteredRadiance, float3 normalIncidenceReflectance, float2 reflectanceIntegral)
{
    return prefilteredRadiance * (normalIncidenceReflectance * reflectanceIntegral.x + reflectanceIntegral.y);
}

// 粗さが指す鏡面畳込みの縮小段の位置。焼く側が段の粗さを段番号を段数から1を引いた数で割った値にしているため、
// その逆写像を取る。整数へ丸めないのは、段の間を補間して標本するためである。
float indirectSpecularMipLevel(float roughness, uint levelCount)
{
    return saturate(roughness) * float(levelCount - 1);
}
