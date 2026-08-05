// 遠方環境の契約の画素段が読む3つの画像の宣言と、そこから表面の間接光の色を返す関数。
// 消費の式そのものは`distant_environment_shading`が持ち、このモジュールは「どの画像をどの向きで標本するか」だけを担う。
// 3つの画像は照明問い合わせのセット(set3)のbinding4から6にあり、定数近似の契約のパイプラインは1つも宣言しない。
// 注意: 拡散照度と鏡面畳込みは太陽相対座標で焼かれているため、標本する前にワールドの向きを同じ座標へ写す。
// 写さずにワールドの向きのまま標本すると、太陽の方位が回るぶんだけ間接光の向きがずれる。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import distant_environment_shading;
import local_visibility_apply;
import sun_relative_frame;

[[vk::binding(4, 3)]] [[vk::combinedImageSampler]] TextureCube diffuseIrradianceTexture;
[[vk::binding(4, 3)]] [[vk::combinedImageSampler]] SamplerState diffuseIrradianceSampler;
[[vk::binding(5, 3)]] [[vk::combinedImageSampler]] TextureCube specularPrefilterTexture;
[[vk::binding(5, 3)]] [[vk::combinedImageSampler]] SamplerState specularPrefilterSampler;
[[vk::binding(6, 3)]] [[vk::combinedImageSampler]] Texture2D reflectanceIntegralTexture;
[[vk::binding(6, 3)]] [[vk::combinedImageSampler]] SamplerState reflectanceIntegralSampler;

// 鏡面畳込みが持つ粗さ段の数。段の数はCPU側の解像度が決めて画像の縮小段として現れるため、
// 定数を別に持たず画像そのものへ問い合わせる。
uint distantEnvironmentSpecularLevelCount()
{
    uint width;
    uint height;
    uint levelCount;
    specularPrefilterTexture.GetDimensions(0, width, height, levelCount);
    return levelCount;
}

// 法線の向きが受ける拡散照度。Lambertの畳み込み後の照度そのものであり、円周率で割るのは消費側である。
float3 distantEnvironmentIrradiance(float3 sunRelativeNormal)
{
    return diffuseIrradianceTexture.Sample(diffuseIrradianceSampler, sunRelativeNormal).rgb;
}

// 反射方向と粗さが指す畳み込み後の放射輝度。段の間を補間するため段の位置を実数で渡す。
float3 distantEnvironmentPrefiltered(float3 sunRelativeReflection, float roughness)
{
    float level = indirectSpecularMipLevel(roughness, distantEnvironmentSpecularLevelCount());
    return specularPrefilterTexture.SampleLevel(specularPrefilterSampler, sunRelativeReflection, level).rgb;
}

// 分割和の第2項。表の横軸が法線と視線の余弦、縦軸が粗さであり、どちらもそのまま標本座標になる。
float2 distantEnvironmentReflectanceIntegral(float nDotV, float roughness)
{
    return reflectanceIntegralTexture.Sample(reflectanceIntegralSampler, float2(nDotV, roughness)).rg;
}

// 標準PBRの表面が受け取る間接光の色。拡散と鏡面を足した値を返す。
// 法線と視線の向きはワールドであり、太陽方向もワールドである。
// 局所可視度は拡散の項だけへ掛かる。鏡面は反射方向の細い立体角を見るため、半球全体の塞がり具合を掛ける根拠が無い。
float3 distantEnvironmentIndirect(
    float3 worldNormal, float3 worldViewDir, float3 sunDirection,
    float3 baseColor, float metallic, float roughness, float localVisibility)
{
    float3 normal = normalize(worldNormal);
    float3 viewDir = normalize(worldViewDir);
    float nDotV = saturate(dot(normal, viewDir));
    SunRelativeFrame frame = sunRelativeFrameOf(sunDirection);
    float3 sunRelativeNormal = sunRelativeOfWorld(frame, normal);
    float3 sunRelativeReflection = sunRelativeOfWorld(frame, reflect(-viewDir, normal));

    float3 normalIncidenceReflectance = indirectNormalIncidenceReflectance(baseColor, metallic);
    float3 fresnel = indirectRoughnessFresnel(nDotV, normalIncidenceReflectance, roughness);
    float3 diffuse = indirectDiffuseColor(
        indirectDiffuseRatio(fresnel, metallic), distantEnvironmentIrradiance(sunRelativeNormal), baseColor);
    float3 specular = indirectSpecularColor(
        distantEnvironmentPrefiltered(sunRelativeReflection, roughness), normalIncidenceReflectance,
        distantEnvironmentReflectanceIntegral(nDotV, roughness));
    return applyLocalVisibility(diffuse, localVisibility) + specular;
}

// 布の表面が受け取る間接光の色。布は金属でないため取り分を掛けず、拡散照度をそのままLambertの規約で割る
// (参照: `_doc/設計/放射輝度問い合わせ階層.md`「世界の間接照明方針と契約の2枝(3-Ic)」)。
// 返すのはベースカラーを掛ける前の値であり、掛けるのは呼び出し元である。
float3 distantEnvironmentClothIndirect(float3 worldNormal, float3 sunDirection)
{
    SunRelativeFrame frame = sunRelativeFrameOf(sunDirection);
    float3 sunRelativeNormal = sunRelativeOfWorld(frame, normalize(worldNormal));
    return distantEnvironmentIrradiance(sunRelativeNormal) / 3.14159265;
}
