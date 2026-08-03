// 立方体の面の中の位置と向きとの行き来。軸の割り当てと符号はVulkanの立方体画像の面選択の規約に合わせる。
// 注意: この写像は`crates/blitz_render/src/atmosphere/mapping/cube_face_mapping.rs`の写しである。
// 正本はRust側にあり、往復が恒等であることのユニットテストもそちらが持つ。ここが正本から外れると、
// 焼いた面と参照する面が食い違って絵が面ごとに入れ替わる。
// 注意: シェーダー内識別子はASCII(CLAUDE.md「命名」)。

// 面と面内位置から向きを求める。面内位置は横縦とも0以上1以下であり、0が面の左端・上端に当たる。
// 層番号の並びは0から順に第一軸の正・負、第二軸の正・負、第三軸の正・負である。
float3 directionOfCubeFace(uint faceLayer, float2 facePosition)
{
    float u = 2.0 * facePosition.x - 1.0;
    float v = 2.0 * facePosition.y - 1.0;
    float3 raw;
    switch (faceLayer)
    {
    case 0:
        raw = float3(1.0, -v, -u);
        break;
    case 1:
        raw = float3(-1.0, -v, u);
        break;
    case 2:
        raw = float3(u, 1.0, v);
        break;
    case 3:
        raw = float3(u, -1.0, -v);
        break;
    case 4:
        raw = float3(u, -v, 1.0);
        break;
    default:
        raw = float3(-u, -v, -1.0);
        break;
    }
    return normalize(raw);
}

// テクセルの添字を、そのテクセルの中心が指す面内位置へ写す。焼く側が使う。
float2 cubeFaceTexelCenter(uint2 texel, uint faceSide)
{
    return (float2(texel) + 0.5) / float(faceSide);
}
