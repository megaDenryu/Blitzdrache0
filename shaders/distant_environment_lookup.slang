// 遠方環境の立方体画像から、向き1本が指すテクセルを1つだけ読む。
// 補間せず1テクセルだけを読むのは、CPUの正本とこの写しが同じ値を返すことを最優先するためである。
// ハードウェアの立方体の補間は面の境界をまたいで混ぜるため、CPU側で同じ結果を再現できず、
// 写しの誤りと補間の違いを見分けられなくなる。畳み込みが広い立体角にわたって多数の標本を平均するため、
// 1テクセルずつ読んでも結果のむらは標本数で押さえられる。
// 注意: この参照は`crates/blitz_render/src/distant_environment/derived/cube_texel_lookup.rs`の写しである。正本はRust側にある。
// 注意: 立方体ビューでなく2次元配列ビューを受け取るのは、層番号を自分で決めて読むためである。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

import cube_face_mapping;

float3 distantEnvironmentTexel(Texture2DArray<float4> distantEnvironment, float3 direction)
{
    uint width;
    uint height;
    uint layers;
    distantEnvironment.GetDimensions(width, height, layers);
    CubeFacePosition found = cubeFaceOfDirection(direction);
    uint2 texel = cubeFaceTexelOfPosition(found.position, width);
    return distantEnvironment.Load(int4(int(texel.x), int(texel.y), int(found.faceLayer), 0)).rgb;
}
